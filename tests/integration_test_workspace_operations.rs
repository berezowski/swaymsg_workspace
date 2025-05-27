mod common;
use swaymsg_workspace::commands::Command;

#[test]
fn test_selecting_workspace() {
	// select next workspace on same screen
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(workspaces, Command::Next, None);
	let expected = "workspace '1 Foo'; focus child";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);

	// select next workspace on same screen (wrap to start)
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(workspaces, Command::Next, None);
	let expected = "workspace '1 Foo'; focus child";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);

	// select next workspace on same screen (wrap to start)
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result =
		swaymsg_workspace::execute_userinput(workspaces, Command::Number, Some("1".to_string()));
	let expected = "workspace '1 Foo'; focus child";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);
}

#[test]
fn test_moving_workspace() {
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(workspaces, Command::SwapWithNext, None);
	let expected = "rename workspace '2 Bar' to '1 Bar\u{200b}' | rename workspace '1 Foo' to '2 Foo' | rename workspace '1 Bar\u{200b}' to '1 Bar'";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);

	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(workspaces, Command::SwapWithPrev, None);
	let expected = "rename workspace '1 Foo' to '2 Foo' | rename workspace '2 Bar' to '1 Bar\u{200b}' | rename workspace '1 Bar\u{200b}' to '1 Bar'";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);
}

#[test]
fn test_reindexing_workspace() {
	// reindex to a free workspace number
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(workspaces, Command::Increase, None);
	let expected = "rename workspace '2 Bar' to '3 Bar'";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);

	// reindex to a already taken workspace number
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(workspaces, Command::Decrease, None);
	let expected = "rename workspace '1 Foo' to '2 Foo' | rename workspace '2 Bar' to '1 Bar\u{200b}' | rename workspace '1 Bar\u{200b}' to '1 Bar'";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);
}

#[test]
fn test_renaming_workspace() {
	// rename to non existent name Baz
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(
	                                                  workspaces,
	                                                  Command::RenameTo,
	                                                  Some("Baz".to_string()),
	);
	let expected = "rename workspace '2 Bar' to '2 Baz'";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);

	// rename to existing name Foo
	let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
	let result = swaymsg_workspace::execute_userinput(
	                                                  workspaces,
	                                                  Command::RenameTo,
	                                                  Some("Foo".to_string()),
	);
	let expected = "rename workspace '2 Bar' to '2 Foo'";
	assert_eq!(&result.ok().unwrap().join(" | "), &expected);
	assert_eq!(&commandhistory.take().join(" | "), &expected);
}
