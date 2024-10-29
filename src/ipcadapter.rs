use core::fmt::Debug;
use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////
// INTERFACES
////////////////////////////////////////////////////////////////////////////////

pub type IpcResult = Result<Vec<String>, Box<dyn Error>>;

use std::fmt;

#[derive(Debug)]
pub struct IpcError {
    pub command: String,
}

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Command {} failed", self.command)
    }
}

pub trait ConnectionProxy {
    fn run_command(&mut self, payload: String) -> IpcResult;
    fn as_any(&self) -> &dyn Any;
}
impl Debug for dyn ConnectionProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionProxy")
    }
}

pub trait WorkspaceProxy {
    fn get_num(&self) -> Option<usize>;
    fn get_name(&self) -> &str;
    fn get_output(&self) -> &str;
    fn get_focused(&self) -> bool;
}
impl Debug for dyn WorkspaceProxy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "WorkspaceProxy{{ name: {} num: {} output: {}  focused: {} }}",
            self.get_name(),
            self.get_num().unwrap(),
            self.get_output(),
            self.get_focused()
        )
    }
}

pub trait OutputProxy {
    fn get_name(&self) -> &str;
    fn get_focused(&self) -> bool;
}

impl Debug for dyn OutputProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OutputProxy{{ name: {} focused: {} }}",
            self.get_name(),
            self.get_focused()
        )
    }
}

pub trait IPCAdapter {
    ///
    /// asdfasdfasfdafdsasdf
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// ```
    fn new() -> impl IPCAdapter;
    fn explode(
        self,
    ) -> (
        RefCell<Box<dyn ConnectionProxy>>,
        Vec<Box<dyn WorkspaceProxy>>,
        Vec<Box<dyn OutputProxy>>,
    );
}

////////////////////////////////////////////////////////////////////////////////
// TESTING IMPLEMENTATION
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MockConnection {
    pub commandhistory: Rc<RefCell<Vec<String>>>,
}

impl ConnectionProxy for MockConnection {
    fn run_command(&mut self, payload: String) -> IpcResult {
        self.commandhistory.borrow_mut().push(payload.clone());
        Ok(vec![payload])
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct MockWorkspace {
    pub num: Option<usize>,
    pub name: String,
    pub output: String,
    pub focused: bool,
}

impl WorkspaceProxy for MockWorkspace {
    fn get_num(&self) -> Option<usize> {
        self.num
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_output(&self) -> &str {
        &self.output
    }
    fn get_focused(&self) -> bool {
        self.focused
    }
}

#[derive(Debug)]
pub struct MockOutput {
    pub name: String,
    pub focused: bool,
}
impl OutputProxy for MockOutput {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_focused(&self) -> bool {
        self.focused
    }
}

pub struct MockIPCAdapter {
    pub connection: RefCell<Box<dyn ConnectionProxy>>,
    pub workspaces: Vec<Box<dyn WorkspaceProxy>>,
    pub outputs: Vec<Box<dyn OutputProxy>>,
}

impl IPCAdapter for MockIPCAdapter {
    fn new() -> impl IPCAdapter {
        MockIPCAdapter {
            connection: RefCell::new(Box::new(MockConnection {
                commandhistory: Rc::new(vec![].into()),
            })),
            workspaces: vec![
                Box::new(MockWorkspace {
                    num: Some(1),
                    name: "Foo".to_string(),
                    output: "eDP-1".to_string(),
                    focused: false,
                }),
                Box::new(MockWorkspace {
                    num: Some(2),
                    name: "Bar".to_string(),
                    output: "eDP-1".to_string(),
                    focused: false,
                }),
                Box::new(MockWorkspace {
                    num: Some(3),
                    name: "Bar".to_string(),
                    output: "eDP-1".to_string(),
                    focused: false,
                }),
                Box::new(MockWorkspace {
                    num: Some(3),
                    name: "Bar".to_string(),
                    output: "HDMI-1".to_string(),
                    focused: false,
                }),
                Box::new(MockWorkspace {
                    num: None,
                    name: "Span2".to_string(),
                    output: "eDP-1".to_string(),
                    focused: false,
                }),
                Box::new(MockWorkspace {
                    num: None,
                    name: "Span2".to_string(),
                    output: "HDMI-1".to_string(),
                    focused: false,
                }),
                Box::new(MockWorkspace {
                    num: None,
                    name: "Span3".to_string(),
                    output: "eDP-1".to_string(),
                    focused: true,
                }),
                Box::new(MockWorkspace {
                    num: None,
                    name: "Span3".to_string(),
                    output: "HDMI-1".to_string(),
                    focused: false,
                }),
                Box::new(MockWorkspace {
                    num: None,
                    name: "Span3".to_string(),
                    output: "HDMI-2".to_string(),
                    focused: true,
                }),
            ],
            outputs: vec![
                Box::new(MockOutput {
                    focused: false,
                    name: "eDP-1".to_string(),
                }),
                Box::new(MockOutput {
                    focused: true,
                    name: "HDMI-1".to_string(),
                }),
                Box::new(MockOutput {
                    focused: false,
                    name: "HDMI-2".to_string(),
                }),
            ],
        }
    }
    fn explode(
        self,
    ) -> (
        RefCell<Box<dyn ConnectionProxy>>,
        Vec<Box<dyn WorkspaceProxy>>,
        Vec<Box<dyn OutputProxy>>,
    ) {
        (self.connection, self.workspaces, self.outputs)
    }
}

////////////////////////////////////////////////////////////////////////////////
// PROD IMPLEMENTATION
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct SwayConnection {
    connection: RefCell<swayipc::Connection>,
}

impl ConnectionProxy for SwayConnection {
    fn run_command(&mut self, payload: String) -> IpcResult {
        match self.connection.borrow_mut().run_command(payload.clone()) {
            Ok(_) => Ok(vec![payload]),
            Err(e) => Err(Box::new(e)),
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct SwayWorkspaceProxy {
    sway_workspace: swayipc::Workspace,
}

impl SwayWorkspaceProxy {
    fn new(sway_workspace: swayipc::Workspace) -> Box<dyn WorkspaceProxy> {
        Box::new(SwayWorkspaceProxy { sway_workspace })
    }
}

impl WorkspaceProxy for SwayWorkspaceProxy {
    fn get_num(&self) -> Option<usize> {
        if self.sway_workspace.num > 0 {
            Some(self.sway_workspace.num as usize)
        } else {
            None
        }
    }
    fn get_name(&self) -> &str {
        &self.sway_workspace.name
    }
    fn get_output(&self) -> &str {
        &self.sway_workspace.output
    }
    fn get_focused(&self) -> bool {
        self.sway_workspace.focused
    }
}

#[derive(Debug)]
struct SwayOutputProxy {
    sway_output: swayipc::Output,
}

impl SwayOutputProxy {
    fn new(sway_output: swayipc::Output) -> Box<dyn OutputProxy> {
        Box::new(SwayOutputProxy { sway_output })
    }
}

impl OutputProxy for SwayOutputProxy {
    fn get_name(&self) -> &str {
        &self.sway_output.name
    }
    fn get_focused(&self) -> bool {
        self.sway_output.focused
    }
}

#[derive(Debug)]
pub struct SwayIPCAdapter {
    connection: SwayConnection,
    workspaces: Vec<Box<dyn WorkspaceProxy>>,
    outputs: Vec<Box<dyn OutputProxy>>,
}

impl IPCAdapter for SwayIPCAdapter {
    fn new() -> impl IPCAdapter {
        if let Some(mut connection) = swayipc::Connection::new().ok() {
            match (
                swayipc::Connection::get_workspaces(&mut connection),
                swayipc::Connection::get_outputs(&mut connection),
            ) {
                (Ok(workspaces), Ok(outputs)) => SwayIPCAdapter {
                    connection: SwayConnection {
                        connection: RefCell::new(connection),
                    },
                    workspaces: workspaces
                        .iter()
                        .map(|workspace| SwayWorkspaceProxy::new(workspace.to_owned()))
                        .collect::<Vec<Box<dyn WorkspaceProxy>>>(),
                    outputs: outputs
                        .iter()
                        .map(|output| SwayOutputProxy::new(output.to_owned()))
                        .collect::<Vec<Box<dyn OutputProxy>>>(),
                },
                _ => panic!("Got no Workspaces or Outputs from IPC Connection"),
            }
        } else {
            panic!("IPC Connection failed");
        }
    }
    fn explode(
        self,
    ) -> (
        RefCell<Box<dyn ConnectionProxy>>,
        Vec<Box<dyn WorkspaceProxy>>,
        Vec<Box<dyn OutputProxy>>,
    ) {
        (
            RefCell::new(Box::new(self.connection)),
            self.workspaces,
            self.outputs,
        )
    }
}
