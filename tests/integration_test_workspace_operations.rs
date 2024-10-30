mod common;

#[test]
fn test_selecting_workspace() {
    // select next workspace on same screen
    let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
    let result = swaymsg_workspace::execute_userinput(workspaces, String::from("next"), None);
    let expected = "workspace '1 Foo'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);

    // select next workspace on same screen (wrap to start)
    let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
    let result = swaymsg_workspace::execute_userinput(workspaces, String::from("prev"), None);
    let expected = "workspace '1 Foo'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);

    // select next workspace on same screen (wrap to start)
    let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
    let result = swaymsg_workspace::execute_userinput(
        workspaces,
        String::from("number"),
        Some("1".to_string()),
    );
    let expected = "workspace '1 Foo'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);
}

#[test]
fn test_moving_workspace() {
    let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
    let result =
        swaymsg_workspace::execute_userinput(workspaces, String::from("swap_with_next"), None);
    let expected = "rename workspace '2 Bar' to '1 Bar\u{200b}' | rename workspace '1 Foo' to '2 Foo' | rename workspace '1 Bar\u{200b}' to '1 Bar'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);

    let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
    let result =
        swaymsg_workspace::execute_userinput(workspaces, String::from("swap_with_prev"), None);
    let expected = "rename workspace '1 Foo' to '2 Foo' | rename workspace '2 Bar' to '1 Bar\u{200b}' | rename workspace '1 Bar\u{200b}' to '1 Bar'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);
}

#[test]
fn test_reindexing_workspace() {
    // reindex to a free workspace number
    let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
    let result = swaymsg_workspace::execute_userinput(workspaces, String::from("increase"), None);
    let expected = "rename workspace '2 Bar' to '3 Bar'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);

    // reindex to a already taken workspace number
    let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
    let result = swaymsg_workspace::execute_userinput(workspaces, String::from("decrease"), None);
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
        String::from("rename_to"),
        Some("Baz".to_string()),
    );
    let expected = "rename workspace '2 Bar' to '2 Baz'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);

    // rename to existing name Foo
    let (workspaces, commandhistory) = common::setup_4_workspaces_across_3_outputs();
    let result = swaymsg_workspace::execute_userinput(
        workspaces,
        String::from("rename_to"),
        Some("Foo".to_string()),
    );
    let expected = "rename workspace '2 Bar' to '2 Foo'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);
}
