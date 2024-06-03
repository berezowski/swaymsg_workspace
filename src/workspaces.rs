use core::panic;
use regex::Regex;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use swayipc::Connection;

#[derive(Debug)]
pub struct Workspace {
    number: RefCell<Option<usize>>,
    name: RefCell<Option<String>>,
    pub basename: String,
    workspaces: RefCell<Weak<Workspaces>>,
}

#[derive(Debug)]
pub struct Workspaces {
    focused_index: usize,
    connection: RefCell<Connection>,
    taints: RefCell<Vec<(String, String)>>,
    pub active_workspaces: Vec<Workspace>,
    pub inactive_workspaces: Vec<Workspace>,
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

    pub fn slice_basename<'a>(&'a self) -> (usize, &'a str) {
        self.cast_and_validate_fragments(self.extract_fragments(&self.basename))
    }

    fn cast_and_validate_fragments<'a>(
        &'a self,
        fragments: (&'a str, &'a str),
    ) -> (usize, &'a str) {
        let (number, name) = fragments;
        dbg!(&number);
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
        if let Ok(capture_starting_number) = Regex::new(r"^(?P<number>(\d*)).*") {
            for workspace in self
                .workspaces
                .borrow()
                .upgrade()
                .unwrap()
                .active_workspaces
                .iter()
                .rev()
            {
                dbg!(&workspace);
                if let Some(caps) = capture_starting_number.captures(&workspace.basename) {
                    if let Ok(number) = &caps["number"].parse::<usize>() {
                        return (*number + 1) as usize;
                    };
                }
            }
        }
        1 // default if no other workspace is enumerated
    }
}

impl Workspaces {
    pub fn focused<'a>(&'a self) -> &'a Workspace {
        self.active_workspaces.get(self.focused_index).unwrap()
    }
    pub fn workspace<'a>(&'a self, active_index: usize) -> Option<&'a Workspace> {
        self.active_workspaces.get(active_index)
    }
    pub fn focused_index(&self) -> usize {
        self.focused_index
    }
    pub fn new() -> Rc<Workspaces> {
        if let Some(mut connection) = Connection::new().ok() {
            match (
                Connection::get_workspaces(&mut connection),
                Connection::get_outputs(&mut connection),
            ) {
                (Ok(mut ipcworkspaces), Ok(ipcoutputs)) => {
                    ipcworkspaces.sort_by(|a, b| a.name.cmp(&b.name));

                    let focused_output_name = ipcoutputs
                        .iter()
                        .filter(|output| output.focused)
                        .map(|output| output.name.to_owned())
                        .last()
                        .unwrap();

                    let inactive_workspaces = ipcworkspaces
                        .iter()
                        .filter(|workspace| workspace.output != focused_output_name)
                        .map(|workspace| Workspace {
                            number: RefCell::new(None),
                            name: RefCell::new(None),
                            basename: workspace.name.clone(),
                            workspaces: RefCell::new(Weak::new()),
                        })
                        .collect::<Vec<Workspace>>();

                    let mut focused_index: usize = 0;
                    let active_workspaces = ipcworkspaces
                        .iter()
                        .filter(|workspace| workspace.output == focused_output_name)
                        .enumerate()
                        .map(|workspace| {
                            if workspace.1.focused == true {
                                focused_index = workspace.0
                            };
                            Workspace {
                                number: RefCell::new(None),
                                name: RefCell::new(None),
                                basename: workspace.1.name.clone(),
                                workspaces: RefCell::new(Weak::new()),
                            }
                        })
                        .collect::<Vec<Workspace>>();

                    let workspaces = Rc::new(Workspaces {
                        connection: RefCell::new(connection),
                        taints: RefCell::new(vec![]),
                        active_workspaces,
                        inactive_workspaces,
                        focused_index,
                    });

                    workspaces
                        .active_workspaces
                        .iter()
                        .chain(workspaces.inactive_workspaces.iter())
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
            self.rename(
                &ws1.basename,
                format!("{} {}", ws2.get_number(), ws1.get_name(),).trim(),
            );
            self.rename(
                &ws2.basename,
                format!("{} {}", ws1.get_number(), ws2.get_name()).trim(),
            );
        }
    }
    pub fn increase_index(&self, ws: &Workspace) {
        self.rename(
            &ws.basename,
            format!("{} {}", ws.get_number() + 1, ws.get_name()).trim(),
        );
    }
    pub fn decrease_index(&self, ws: &Workspace) {
        if ws.get_number() > 1 {
            self.rename(
                &ws.basename,
                format!("{} {}", ws.get_number() - 1, ws.get_name()).trim(),
            )
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
            .active_workspaces
            .iter()
            .chain(self.inactive_workspaces.iter())
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
    pub fn move_window(&self, to: &String) {
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
    pub fn cleanup(&self) {
        let mut connection = self.connection.borrow_mut();
        for taint in self.taints.borrow_mut().iter() {
            let _ =
                connection.run_command(format!("rename workspace '{}' to '{}'", taint.1, taint.0));
        }
    }
}
