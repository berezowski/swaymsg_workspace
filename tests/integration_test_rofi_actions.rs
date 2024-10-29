mod common;

#[test]
fn test_moving_windows_via_rofi() {
    // missing 'to' parameter
    let (workspaces, commandhistory) = common::setup_4workspaces_across_3outputs();
    let result =
        swaymsg_workspace::execute_userinput(workspaces, String::from("rofi_move_window"), None);
    assert_eq!(result.ok().unwrap().concat(), "nonipc: rofi_move_window");
    assert_eq!(commandhistory.take().concat(), "");

    // move window to existing Foo
    let (workspaces, commandhistory) = common::setup_4workspaces_across_3outputs();
    let result = swaymsg_workspace::execute_userinput(
        workspaces,
        String::from("rofi_move_window"),
        Some(String::from("1 Foo")),
    );
    let expected = "move window to workspace '1 Foo'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);

    // move window to new workspace with a shared name but different basename (Foo's already taken by 1)
    let (workspaces, commandhistory) = common::setup_4workspaces_across_3outputs();
    let result = swaymsg_workspace::execute_userinput(
        workspaces,
        String::from("rofi_move_window"),
        Some(String::from("Foo")),
    );
    let expected = "move window to workspace '3 Foo'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);

    // move window to non existing IDONOTEXISTYET
    let (workspaces, commandhistory) = common::setup_4workspaces_across_3outputs();
    let result = swaymsg_workspace::execute_userinput(
        workspaces,
        String::from("rofi_move_window"),
        Some(String::from("IDONOTEXISTYET")),
    );
    let expected = "move window to workspace '3 IDONOTEXISTYET'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);
}
