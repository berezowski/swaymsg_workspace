mod common;
use swaymsg_workspace::commands::Command;

#[test]
fn test_rofi_move_window_() {
	// missing 'to' parameter
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(workspaces, Command::RofiMoveWindow, None);
	assert_eq!(result.ok().unwrap().concat(), "nonipc: rofi_move_window");
	assert_eq!(commandhistory.take().concat(), "");
}

#[test]
fn test_rofi_move_window_1_foo() {
	// move window to existing Foo
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(
	                                                  workspaces,
	                                                  Command::RofiMoveWindow,
	                                                  Some(String::from("1 Foo")),
	);
	let expected = "move window to workspace '1 Foo'";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);
}

#[test]
fn test_rofi_move_window_foo() {
	// move window to new workspace with a shared name but different basename (Foo's already taken by 1)
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(
	                                                  workspaces,
	                                                  Command::RofiMoveWindow,
	                                                  Some(String::from("Foo")),
	);
	let expected = "move window to workspace '3 Foo'";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);
}

#[test]
fn test_rofi_move_window_idonotexistyet() {
	// move window to non existing IDONOTEXISTYET
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(
	                                                  workspaces,
	                                                  Command::RofiMoveWindow,
	                                                  Some(String::from("IDONOTEXISTYET")),
	);
	let expected = "move window to workspace '3 IDONOTEXISTYET'";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);
}
