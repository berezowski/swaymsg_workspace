use std::{cell::RefCell, rc::Rc};

use swaymsg_workspace::{
	ipcadapter::{ConnectionProxy, MockConnection, MockIPCAdapter, MockOutput, MockWorkspace},
	workspaces::Workspaces,
};

pub fn setup_4_workspaces_across_3_outputs() -> (Rc<Workspaces>, Rc<RefCell<Vec<String>>>) {
	let commands = Rc::new(RefCell::new(vec![]));
	let connection: RefCell<Box<dyn ConnectionProxy>> = RefCell::new(Box::new(MockConnection {
		commandhistory: commands.clone(),
	}));
	let ipcadapter = MockIPCAdapter {
		connection,
		workspaces: vec![
			Box::new(MockWorkspace {
				num: Some(1),
				name: "1 Foo".to_string(),
				output: "eDP-1".to_string(),
				focused: false,
			}),
			Box::new(MockWorkspace {
				num: Some(2),
				name: "2 Bar".to_string(),
				output: "eDP-1".to_string(),
				focused: true,
			}),
			Box::new(MockWorkspace {
				num: Some(2),
				name: "1 Bar".to_string(),
				output: "HDMI-1".to_string(),
				focused: false,
			}),
			Box::new(MockWorkspace {
				num: Some(2),
				name: "Bak".to_string(),
				output: "HDMI-2".to_string(),
				focused: false,
			}),
		],
		outputs: vec![
			Box::new(MockOutput {
				focused: true,
				name: "eDP-1".to_string(),
			}),
			Box::new(MockOutput {
				focused: false,
				name: "HDMI-1".to_string(),
			}),
			Box::new(MockOutput {
				focused: false,
				name: "HDMI-2".to_string(),
			}),
		],
	};

	(Workspaces::new(ipcadapter), commands)
}
