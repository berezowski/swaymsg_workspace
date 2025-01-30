use std::env;
use swaymsg_workspace::{execute_userinput, Command};
pub mod ipcadapter;
pub mod workspaces;

use swaymsg_workspace::ipcadapter::{IPCAdapter, SwayIPCAdapter};
use swaymsg_workspace::workspaces::Workspaces;

fn main() {
	let ipcadapter = SwayIPCAdapter::new();
	let workspaces = Workspaces::new(ipcadapter);

	// split args by ' ' to handle the combined argument which rofi supplies
	let mut args =
		env::args().collect::<Vec<String>>()
		           .into_iter()
		           .flat_map(|arg| arg.split(' ').map(str::to_owned).collect::<Vec<String>>())
		           .into_iter();

	if let Some(main_argument) = args.nth(1) {
		let command_from_argument = match main_argument.as_str() {
			"next" => Command::Next,
			"prev" => Command::Prev,
			"swap_with_prev" => Command::SwapWithPrev,
			"swap_with_next" => Command::SwapWithNext,
			"increase" => Command::Increase,
			"decrease" => Command::Decrease,
			"rename_to" => Command::RenameTo,
			"number" => Command::Number,
			"move_container_to_workspace_number" => Command::MoveContainerToWorkspaceNumber,
			"select" => Command::Select,
			"print_focused_name" => Command::PrintFocusedName,
			"print_focused_number" => Command::PrintFocusedNumber,
			"rofi_select_workspace" => Command::RofiSelectWorkspace,
			"rofi_move_window" => Command::RofiMoveWindow,
			_ => Command::Usage,
		};

		if let Err(error) = execute_userinput(
		                                      workspaces,
		                                      command_from_argument,
		                                      args.reduce(|a, b| format!("{} {}", a, b)), // parameters to argument
		) {
			eprintln!("Something broke: {error}");
		}
	} else {
		eprintln!("usage: swaymsg_workspace [ next prev swap_with_prev swap_with_next increase decrease rename_to select print_focused_name print_focused_number rofi_select_workspace rofi_move_window ]. ");
	}
}
