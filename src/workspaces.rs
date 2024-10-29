use regex::{Error, Regex};
use std::cmp::Ordering;
use std::ops::Deref;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::ipcadapter::{ConnectionProxy, IPCAdapter, IpcResult, OutputProxy, WorkspaceProxy};

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
    same_screen_focused_index: usize,
    connection: RefCell<Box<dyn ConnectionProxy>>,
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

    pub fn rename(&self, to: &str) -> IpcResult {
        self.workspaces().rename(&self.basename, to)
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
    fn workspaces(&self) -> Rc<Workspaces> {
        self.workspaces.borrow().upgrade().unwrap()
    }
    fn find_free_workspace_num(&self) -> usize {
        find_free_workspace_num(&self.workspaces().on_same_screen)
    }
}

pub fn find_free_workspace_num(workspaces: &VecOfWorkspaces) -> usize {
    workspaces
            .iter()
            .rev() // since workspaces are sorted we start from the back to get the highest number
            .filter_map(|workspace| extract_starting_number(&workspace.basename))
            .next()
            .unwrap_or_default() // 0 if no other workspace is enumerated
            + 1 // remember, we want the next free number
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
    workspace1: &Box<dyn WorkspaceProxy>,
    workspace2: &Box<dyn WorkspaceProxy>,
) -> std::cmp::Ordering {
    match (
        extract_starting_number(&workspace1.get_name()),
        extract_starting_number(&workspace2.get_name()),
    ) {
        (Some(number1), Some(number2)) => match number1.cmp(&number2) {
            Ordering::Equal => workspace1.get_name().cmp(&workspace2.get_name()),
            ordering => ordering,
        },
        (Some(_num), None) => Ordering::Greater,
        (None, Some(_num)) => Ordering::Less,
        (None, None) => workspace1.get_name().cmp(&workspace2.get_name()),
    }
}

impl Workspaces {
    pub fn new(source: impl IPCAdapter) -> Rc<Workspaces> {
        Workspaces::extract_workspaces(source.explode())
    }

    fn run_command(&self, command: String) -> IpcResult {
        self.connection.borrow_mut().run_command(command)
    }

    fn map_workspaceproxy_to_workspace(proxy: &Box<dyn WorkspaceProxy>) -> Workspace {
        Workspace {
            number: RefCell::new(proxy.get_num()),
            name: RefCell::new({
                Some(
                    proxy
                        .get_name()
                        .trim_start_matches(&proxy.get_num().unwrap_or_default().to_string())
                        .trim()
                        .to_string(),
                )
            }),
            basename: proxy.get_name().to_string(),
            workspaces: RefCell::new(Weak::new()),
            focused: proxy.get_focused(),
        }
    }
    pub fn extract_workspaces(
        (connection, mut ipcworkspaces, ipcoutputs): (
            RefCell<Box<dyn ConnectionProxy>>,
            Vec<Box<dyn WorkspaceProxy>>,
            Vec<Box<dyn OutputProxy>>,
        ),
    ) -> Rc<Workspaces> {
        ipcworkspaces.sort_by(|a, b| sort_ipcworkspace(a, b));
        let focused_output_name = ipcoutputs
            .iter()
            .filter(|output| output.get_focused())
            .map(|output| output.get_name())
            .last()
            .unwrap();

        let workspaces_on_other_screen = ipcworkspaces
            .iter()
            .filter(|ipc_workspace| ipc_workspace.get_output() != focused_output_name)
            .map(Self::map_workspaceproxy_to_workspace)
            .collect();

        let mut same_screen_focused_index: usize = 0;
        let workspaces_on_same_screen = ipcworkspaces
            .iter()
            .filter(|ipc_workspace| ipc_workspace.get_output() == focused_output_name)
            .enumerate()
            .map(|(index, ipc_workspace)| {
                if ipc_workspace.get_focused() {
                    same_screen_focused_index = index
                };
                ipc_workspace // remove enumeration from stream
            })
            .map(Self::map_workspaceproxy_to_workspace)
            .collect();

        let workspaces = Rc::new(Workspaces {
            connection: connection,
            taints: RefCell::new(vec![]),
            on_same_screen: workspaces_on_same_screen,
            on_other_screen: workspaces_on_other_screen,
            same_screen_focused_index,
        });

        workspaces
            .on_same_screen
            .iter()
            .chain(workspaces.on_other_screen.iter())
            .for_each(|ws| *ws.workspaces.borrow_mut() = Rc::downgrade(&workspaces));
        return workspaces;
    }
    pub fn get_focused(&self) -> &Workspace {
        self.on_same_screen
            .get(self.same_screen_focused_index as isize)
            .unwrap()
    }
    pub fn focused_index(&self) -> isize {
        self.same_screen_focused_index as isize
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
    pub fn swap(&self, ws1: &Workspace, ws2: &Workspace) -> IpcResult {
        if ws1.basename == ws2.basename {
            return Ok(vec!["".to_string()]);
        }
        if ws1.get_number() == ws2.get_number() {
            self.increase_number(ws1)
        } else {
            ws1.rename(format!("{} {}", ws2.get_number(), ws1.get_name()).trim())
                .and_then(|mut ws1_rename_result| {
                    ws2.rename(format!("{} {}", ws1.get_number(), ws2.get_name()).trim())
                        .and_then(|ws2_rename_result| {
                            ws1_rename_result.extend(ws2_rename_result);
                            Ok(ws1_rename_result)
                        })
                })
        }
    }
    pub fn increase_number(&self, ws: &Workspace) -> IpcResult {
        ws.rename(format!("{} {}", ws.get_number() + 1, ws.get_name()).trim())
    }
    pub fn decrease_number(&self, ws: &Workspace) -> IpcResult {
        if ws.get_number() > 1 {
            ws.rename(format!("{} {}", ws.get_number() - 1, ws.get_name()).trim())
        } else {
            Ok(vec!["".to_string()])
        }
    }
    pub fn rename(&self, from: &str, to: &str) -> IpcResult {
        self.run_command(format!(
            "rename workspace '{from}' to '{}'",
            self.dedupguard(to.to_string().replace("\u{200B}", ""))
        ))
    }
    //
    // Workspaces might temporarily (while swapping e.g.) or permanently (on different
    // outputs) share the same name.
    // To avoid conflicts this method appends a zero width space to the name.
    // This alteration is stored in a kind of history array called 'taints'
    //
    pub fn dedupguard(&self, desired: String) -> String {
        match self
            .on_same_screen
            .iter()
            .chain(self.on_other_screen.iter())
            .find(|ws| ws.basename == desired)
        {
            Some(_) => {
                let to = desired.to_string() + "\u{200B}"; //add 'zero width space' char
                self.taints
                    .borrow_mut()
                    .push((desired.to_string(), to.clone()));
                self.dedupguard(to)
            }
            None => desired,
        }
    }
    pub fn move_container_to(&self, to: &String) -> IpcResult {
        self.run_command(format!("move window to workspace '{}'", to))
    }
    pub fn select(&self, name: &String) -> IpcResult {
        self.run_command(format!("workspace '{}'", name))
    }
    //
    // create and select a new workspace, calling self.dedupguard() is important
    // because a workspace with this name might already exist on another output
    //
    pub fn select_new(&self, basename: String) -> IpcResult {
        self.run_command(format!("workspace '{}'", self.dedupguard(basename)))
    }
    pub fn move_to_new(&self, basename: String) -> IpcResult {
        self.run_command(format!(
            "move window to workspace '{}'",
            self.dedupguard(basename)
        ))
    }
    //
    // Select Workspace starting with number: {number} on the same Screen
    // If no Workspace starts with {number} the method creates and selects a new one
    //
    pub fn select_or_create_number(&self, number: usize) -> IpcResult {
        match self
            .on_same_screen()
            .iter()
            .filter(|ws| ws.basename.starts_with(&format!("{number}")))
            .last()
        {
            Some(workspace) => self.select(&workspace.basename),
            None => self.select_new(format!("{number}")),
        }
    }
    //
    // Move active container the Workspace starting with number: {number} on the same Screen
    //
    pub fn move_container_to_number(&self, number: usize) -> IpcResult {
        match self
            .on_same_screen()
            .iter()
            .filter(|ws| ws.basename.starts_with(&format!("{number}")))
            .last()
        {
            Some(workspace) => self.move_container_to(&workspace.basename),
            _ => self.move_to_new(format!("{number}")),
        }
    }
    //
    // Remove all 'Zero width spaces' that have been added in order to mitigate duplicate identifiers for workspaces
    //
    pub fn cleanup(&self) -> IpcResult {
        let mut results: Vec<String> = vec![];
        for taint in self.taints.borrow_mut().iter() {
            match self.run_command(format!("rename workspace '{}' to '{}'", taint.1, taint.0)) {
                Ok(history) => results.extend(history),
                Err(_) => (), // Might not be fixable due to duplicate Names on different outputs
            };
        }
        Ok(results)
    }
}
