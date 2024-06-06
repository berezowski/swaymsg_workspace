use core::{num, panic};
use regex::Regex;
use std::cmp::Ordering;
use std::ops::Deref;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use swayipc::Connection;

#[derive(Debug)]
pub struct VecOfWorkspaces {
    items: Vec<Rc<Workspace>>,
    focused: Option<Rc<Workspace>>,
}

impl Deref for VecOfWorkspaces {
    type Target = Vec<Rc<Workspace>>;

    fn deref(&self) -> &Vec<Rc<Workspace>> {
        &self.items
    }
}

impl FromIterator<Workspace> for VecOfWorkspaces {
    fn from_iter<I: IntoIterator<Item = Workspace>>(iter: I) -> Self {
        let mut vec_of_workspaces = VecOfWorkspaces {
            items: vec![],
            focused: None,
        };
        for i in iter {
            let rci = Rc::new(i);
            if rci.focused {
                vec_of_workspaces.focused = Some(rci.clone());
            }
            vec_of_workspaces.items.push(rci);
        }
        vec_of_workspaces
    }
}
impl FromIterator<Rc<Workspace>> for VecOfWorkspaces {
    fn from_iter<I: IntoIterator<Item = Rc<Workspace>>>(iter: I) -> Self {
        let mut vec_of_workspaces = VecOfWorkspaces {
            items: vec![],
            focused: None,
        };
        for i in iter {
            if i.focused {
                vec_of_workspaces.focused = Some(i.clone());
            }
            vec_of_workspaces.items.push(i);
        }
        vec_of_workspaces
    }
}

impl VecOfWorkspaces {
    pub fn get(&self, index: isize) -> Option<&Workspace> {
        if index < 0 {
            None
        } else {
            match self.items.get(index as usize) {
                Some(rc) => Some(&*rc),
                None => None,
            }
        }
    }
    pub fn focused(&self) -> Option<&Workspace> {
        match &self.focused {
            Some(rc) => Some(&*rc),
            None => None,
        }
    }
    pub fn first(&self) -> Option<&Workspace> {
        match self.items.first() {
            Some(rc) => Some(&*rc),
            None => None,
        }
    }
    pub fn last(&self) -> Option<&Workspace> {
        match self.items.last() {
            Some(rc) => Some(&*rc),
            None => None,
        }
    }
    pub fn next_of(&self, index: isize) -> Option<&Workspace> {
        self.get(index as isize + 1)
    }
    pub fn prev_of(&self, index: isize) -> Option<&Workspace> {
        if index < 0 {
            None
        } else {
            self.get(index as isize - 1)
        }
    }
}

#[derive(Debug)]
pub struct Workspace {
    number: RefCell<Option<usize>>,
    name: RefCell<Option<String>>,
    pub basename: String,
    workspaces: RefCell<Weak<Workspaces>>,
    focused: bool,
}

#[derive(Debug)]
pub struct Workspaces {
    focused_index: usize,
    connection: RefCell<Connection>,
    taints: RefCell<Vec<(String, String)>>,
    pub on_same_screen: VecOfWorkspaces,
    pub on_other_screen: VecOfWorkspaces,
}

impl Workspace {
    pub fn get_name(&self) -> String {
        self.find_free_workspace_num();
        let name = &mut *self.name.borrow_mut();
        match name {
            Some(name) => name.clone(),
            None => {
                let number = &mut *self.number.borrow_mut();
                let sliced = self.slice_basename();
                *number = Some(sliced.0);
                *name = Some(sliced.1.to_string());
                sliced.1.to_string()
            }
        }
    }

    pub fn get_number(&self) -> usize {
        let number = &mut *self.number.borrow_mut();
        match number {
            Some(num) => *num,
            None => {
                let name = &mut *self.name.borrow_mut();
                let sliced = self.slice_basename();
                *number = Some(sliced.0);
                *name = Some(sliced.1.to_string());
                number.unwrap()
            }
        }
    }

    pub fn rename(&self, to: &str) {
        self.workspaces
            .borrow()
            .upgrade()
            .unwrap()
            .rename(&self.basename, to);
    }

    fn slice_basename<'a>(&'a self) -> (usize, &'a str) {
        self.cast_and_validate_fragments(self.extract_fragments(&self.basename))
    }

    fn cast_and_validate_fragments<'a>(
        &'a self,
        fragments: (&'a str, &'a str),
    ) -> (usize, &'a str) {
        let (number, name) = fragments;
        let number = match number.parse::<usize>() {
            Ok(number) => number,
            _ => self.find_free_workspace_num(),
        };
        let name = name.trim_start().trim_end();
        (number, name)
    }

    fn extract_fragments<'a>(&self, wsname: &'a str) -> (&'a str, &'a str) {
        let capture_number_and_name = Regex::new(r"^(?<number>\d*)\s*(?<name>.*)").unwrap();
        let caps = capture_number_and_name.captures(&wsname).unwrap();
        (
            &wsname[..caps.get(1).unwrap().end()],
            &wsname[caps.get(2).unwrap().start()..],
        )
    }
    fn find_free_workspace_num(&self) -> usize {
        for workspace in self
            .workspaces
            .borrow()
            .upgrade()
            .unwrap()
            .on_same_screen
            .iter()
            .rev()
        {
            if let Some(starting_number) = extract_starting_number(&workspace.basename) {
                return starting_number + 1;
            }
        }
        1 // default if no other workspace is enumerated
    }
}

pub fn extract_starting_number(source: &str) -> Option<usize> {
    if let Ok(capture_starting_number) = Regex::new(r"^(?P<number>(\d*)).*") {
        if let Some(caps) = capture_starting_number.captures(source) {
            if let Ok(number) = &caps["number"].parse::<usize>() {
                return Some(*number);
            }
        }
    }
    None
}

pub fn sort_ipcworkspace(
    workspace1: &swayipc::Workspace,
    workspace2: &swayipc::Workspace,
) -> std::cmp::Ordering {
    match (
        extract_starting_number(&workspace1.name),
        extract_starting_number(&workspace2.name),
    ) {
        (Some(number1), Some(number2)) => match number1.cmp(&number2) {
            Ordering::Equal => workspace1.name.cmp(&workspace2.name),
            ordering => ordering,
        },
        (Some(_num), None) => Ordering::Greater,
        (None, Some(_num)) => Ordering::Less,
        (None, None) => workspace1.name.cmp(&workspace2.name),
    }
}

impl Workspaces {
    pub fn get_focused<'a>(&'a self) -> &'a Workspace {
        self.on_same_screen
            .get(self.focused_index as isize)
            .unwrap()
    }
    pub fn focused_index(&self) -> isize {
        self.focused_index as isize
    }
    pub fn on_same_screen(&self) -> &VecOfWorkspaces {
        &self.on_same_screen
    }
    pub fn on_other_screen(&self) -> &VecOfWorkspaces {
        &self.on_other_screen
    }
    pub fn on_all_screens(&self) -> VecOfWorkspaces {
        self.on_same_screen
            .iter()
            .chain(self.on_other_screen.iter())
            .map(|i| i.clone())
            .collect()
    }
    pub fn new() -> Rc<Workspaces> {
        if let Some(mut connection) = Connection::new().ok() {
            match (
                Connection::get_workspaces(&mut connection),
                Connection::get_outputs(&mut connection),
            ) {
                (Ok(mut ipcworkspaces), Ok(ipcoutputs)) => {
                    ipcworkspaces.sort_by(|a, b| sort_ipcworkspace(&a, &b));

                    let focused_output_name = ipcoutputs
                        .iter()
                        .filter(|output| output.focused)
                        .map(|output| output.name.to_owned())
                        .last()
                        .unwrap();

                    let workspaces_on_other_screen = ipcworkspaces
                        .iter()
                        .filter(|workspace| workspace.output != focused_output_name)
                        .map(|workspace| Workspace {
                            number: RefCell::new(None),
                            name: RefCell::new(None),
                            basename: workspace.name.clone(),
                            workspaces: RefCell::new(Weak::new()),
                            focused: workspace.focused,
                        })
                        .collect();

                    let mut focused_index: usize = 0;
                    let workspaces_on_same_screen = ipcworkspaces
                        .iter()
                        .filter(|workspace| workspace.output == focused_output_name)
                        .enumerate()
                        .map(|(index, workspace)| {
                            if workspace.focused == true {
                                focused_index = index
                            };
                            Workspace {
                                number: RefCell::new(None),
                                name: RefCell::new(None),
                                basename: workspace.name.clone(),
                                workspaces: RefCell::new(Weak::new()),
                                focused: workspace.focused,
                            }
                        })
                        .collect();

                    let workspaces = Rc::new(Workspaces {
                        connection: RefCell::new(connection),
                        taints: RefCell::new(vec![]),
                        on_same_screen: workspaces_on_same_screen,
                        on_other_screen: workspaces_on_other_screen,
                        focused_index,
                    });

                    workspaces
                        .on_same_screen
                        .iter()
                        .chain(workspaces.on_other_screen.iter())
                        .for_each(|ws| *ws.workspaces.borrow_mut() = Rc::downgrade(&workspaces));
                    return workspaces;
                }
                _ => panic!("Got no Workspaces or Outputs from IPC Connection"),
            }
        } else {
            panic!("IPC Connection failed");
        }
    }
    pub fn swap(&self, ws1: &Workspace, ws2: &Workspace) {
        if ws1.get_number() == ws2.get_number() {
            self.increase_index(ws1);
        } else {
            ws1.rename(format!("{} {}", ws2.get_number(), ws1.get_name()).trim());
            ws2.rename(format!("{} {}", ws1.get_number(), ws2.get_name()).trim());
        }
    }
    pub fn increase_index(&self, ws: &Workspace) {
        ws.rename(format!("{} {}", ws.get_number() + 1, ws.get_name()).trim());
    }
    pub fn decrease_index(&self, ws: &Workspace) {
        if ws.get_number() > 1 {
            ws.rename(format!("{} {}", ws.get_number() - 1, ws.get_name()).trim())
        };
    }
    pub fn rename(&self, from: &str, to: &str) {
        let _res = self.connection.borrow_mut().run_command(format!(
            "rename workspace '{from}' to '{}'",
            self.dedupguard(to.to_string())
        ));
    }
    pub fn dedupguard(&self, desired: String) -> String {
        match self
            .on_same_screen
            .iter()
            .chain(self.on_other_screen.iter())
            .find(|ws| ws.basename == desired)
        {
            Some(_) => {
                let to = desired.to_string() + "\u{FEFF}"; //add 'invisible' char
                self.taints
                    .borrow_mut()
                    .push((desired.to_string(), to.clone()));
                self.dedupguard(to)
            }
            None => desired,
        }
    }
    pub fn move_container_to(&self, to: &String) {
        let _ = self
            .connection
            .borrow_mut()
            .run_command(format!("move window to workspace '{}'", to));
    }
    pub fn select(&self, name: &String) {
        let _ = self
            .connection
            .borrow_mut()
            .run_command(format!("workspace '{}'", name));
    }
    pub fn select_new(&self, basename: String) {
        let _ = self
            .connection
            .borrow_mut()
            .run_command(format!("workspace '{}'", self.dedupguard(basename)));
    }
    pub fn move_to_new(&self, basename: String) {
        let _ = self.connection.borrow_mut().run_command(format!(
            "move window to workspace '{}'",
            self.dedupguard(basename)
        ));
    }
    pub fn select_or_create_number(&self, number: usize) {
        match self
            .on_same_screen()
            .iter()
            .filter(|ws| ws.basename.starts_with(format!("{number}").as_str()))
            .last()
        {
            Some(workspace) => self.select(&workspace.basename),
            None => self.select_new(format!("{number}")),
        }
    }
    pub fn move_container_to_number(&self, number: usize) {
        match self
            .on_same_screen()
            .iter()
            .filter(|ws| ws.basename.starts_with(format!("{number}").as_str()))
            .last()
        {
            Some(workspace) => self.move_container_to(&workspace.basename),
            None => self.move_to_new(format!("{number}")),
        }
    }

    pub fn cleanup(&self) {
        let mut connection = self.connection.borrow_mut();
        for taint in self.taints.borrow_mut().iter() {
            let _ =
                connection.run_command(format!("rename workspace '{}' to '{}'", taint.1, taint.0));
        }
    }
}
