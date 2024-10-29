mod common;

#[test]
fn test_moving_container() {
    let (workspaces, commandhistory) = common::setup_4workspaces_across_3outputs();
    let result = swaymsg_workspace::execute_userinput(
        workspaces,
        String::from("move_container_to_workspace_number"),
        Some("1".to_string()),
    );
    let expected = "move window to workspace '1 Foo'";
    assert_eq!(&result.ok().unwrap().join(" | "), &expected);
    assert_eq!(&commandhistory.take().join(" | "), &expected);
}
