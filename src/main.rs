use regex::Regex;
use std::env;
use swayipc::{Connection, Workspace};
pub mod workspace;

use workspace::WorkspaceDetails;

type WS<'a> = WorkspaceDetails<'a>;

fn main() {
    let mut args = env::args();
    if let Some(main_argument) = args.nth(1) {
        if let Some(mut connection) = Connection::new().ok() {
            match (
                Connection::get_workspaces(&mut connection),
                Connection::get_outputs(&mut connection),
            ) {
                (Ok(workspaces), Ok(outputs)) => {
                    let active_output_name = outputs
                        .iter()
                        .filter(|output| output.focused)
                        .last()
                        .unwrap()
                        .name
                        .to_owned();
                    let workspaces_on_active_output = workspaces
                        .iter()
                        .filter(|ws| ws.output == active_output_name)
                        .collect();
                    if let Some(focused_workspace_index) =
                        find_focused_workspace_num(&workspaces_on_active_output)
                    {
                        execute_userinput(
                            &mut connection,
                            &workspaces_on_active_output,
                            focused_workspace_index, // index in workpaces array
                            main_argument,           // main argumen
                            args.reduce(|a, b| format!("{} {}", a, b)), // parameters to argument
                        );
                    }
                }
                _ => (),
            }
        }
    } else {
        println!("valid arguments: [ swap_with_prev swap_with_next increase decrease rename_to print_focused_name print_focused_number ]. ");
    }
}

fn execute_userinput(
    connection: &mut Connection,
    workspaces: &Vec<&Workspace>,
    focused: usize,
    argument: String,
    argument_parameter: Option<String>,
) {
    let free_workspace_index = || find_free_workspace_num(&workspaces);
    if let Some(ws1) = WS::new(&workspaces.get(focused), free_workspace_index) {
        match argument.as_str() {
            "next" => match workspaces.get(focused + 1) {
                Some(workspace) => select(connection, Some(workspace)),
                None => select(connection, workspaces.first()),
            },
            "prev" => {
                if focused > 0 {
                    select(connection, workspaces.get(focused - 1));
                } else {
                    select(connection, workspaces.last());
                }
            }
            "swap_with_next" => swap_workspace(
                connection,
                Some(ws1),
                WS::new(&workspaces.get(focused + 1), free_workspace_index),
            ),
            "swap_with_prev" => {
                if focused > 0 {
                    swap_workspace(
                        connection,
                        Some(ws1),
                        WS::new(&workspaces.get(focused - 1), free_workspace_index),
                    )
                }
            }
            "increase" => match (
                Some(ws1),
                WS::new(&workspaces.get(focused + 1), free_workspace_index),
            ) {
                (Some(ws1), None) => increase_workspace_number(connection, ws1),
                (Some(ws1), Some(ws2)) => {
                    if ws1.number < ws2.number - 1 {
                        increase_workspace_number(connection, ws1);
                    } else {
                        execute_userinput(
                            connection,
                            workspaces,
                            focused,
                            "swap_with_next".to_string(),
                            None,
                        );
                    }
                }
                _ => (),
            },
            "decrease" => {
                if focused > 0 {
                    if let Some(ws2) = WS::new(&workspaces.get(focused - 1), free_workspace_index) {
                        if ws1.number > ws2.number + 1 {
                            decrease_workspace_number(connection, ws1);
                        } else {
                            execute_userinput(
                                connection,
                                workspaces,
                                focused,
                                "swap_with_prev".to_string(),
                                None,
                            );
                        }
                    }
                } else {
                    decrease_workspace_number(connection, ws1);
                }
            }
            "rename_to" => {
                let _ = if let Some(new_name) = argument_parameter {
                    rename_workspace(
                        connection,
                        &ws1.basename,
                        format!("{} {}", ws1.number, new_name),
                    )
                } else {
                    rename_workspace(connection, &ws1.basename, format!("{} {}", ws1.number, ""))
                };
            }
            "print_focused_name" => {
                println!("{}", ws1.name);
            }
            "print_focused_number" => {
                println!("{}", ws1.number);
            }
            _ => println!("valid arguments: [ swap_with_prev swap_with_next increase decrease ]. "),
        }
    }
}

fn select(connection: &mut Connection, workspace: Option<&&swayipc::Workspace>) {
    match workspace {
        Some(workspace) => {
            let _ = connection.run_command(format!("workspace '{}'", workspace.name));
        }
        _ => (),
    };
}

fn increase_workspace_number(connection: &mut Connection, workspace: WS) {
    let _ = rename_workspace(
        connection,
        &workspace.basename,
        format!("{} {}", workspace.number + 1, &workspace.name),
    );
}

fn decrease_workspace_number(connection: &mut Connection, workspace: WS) {
    if workspace.number > 1 {
        let _ = rename_workspace(
            connection,
            &workspace.basename,
            format!("{} {}", workspace.number - 1, &workspace.name),
        );
    }
}

fn swap_workspace(conn: &mut Connection, ws1: Option<WS>, ws2: Option<WS>) {
    match (ws1, ws2) {
        (Some(ws1), Some(ws2)) => {
            if &ws1.name == &ws2.name {
                let _ = conn.run_command(format!(
                    "rename workspace {} to {} .", // make the name availible, use "." to reduce flickering
                    &ws2.basename, &ws2.basename
                ));
                let _ =
                    rename_workspace(conn, &ws1.basename, format!("{} {}", ws2.number, &ws1.name));

                let _ = conn.run_command(format!(
                    "rename workspace {} . to {}",
                    &ws2.basename,
                    format!("{} {}", ws1.number, &ws2.name),
                ));
            } else {
                let _result =
                    rename_workspace(conn, &ws1.basename, format!("{} {}", ws2.number, &ws1.name));
                let _result =
                    rename_workspace(conn, &ws2.basename, format!("{} {}", ws1.number, &ws2.name));
            }
        }
        _ => (),
    }
}

fn rename_workspace(
    conn: &mut Connection,
    from: &str,
    to: String,
) -> Result<Vec<Result<(), swayipc::Error>>, swayipc::Error> {
    conn.run_command(format!("rename workspace '{}' to '{}'", from, to.trim()))
}

// fn unique_name(test: &str, allowed: &str, connection: &mut Connection) -> {
//     workspaces = connection.get_workspaces().iter().filter(|ws| ws.name != name1))
//     for i in [0..50] {
//     }
//     panic!("more than 50 Workspaces not supported..");
// }

fn find_focused_workspace_num(workspaces: &Vec<&Workspace>) -> Option<usize> {
    for (key, ws) in workspaces.iter().enumerate() {
        if ws.focused {
            return Some(key);
        }
    }
    None
}

fn find_free_workspace_num(workspaces: &Vec<&Workspace>) -> u32 {
    if let Ok(capture_starting_number) = Regex::new(r"^(?P<number>(\d*)).*") {
        for ws in workspaces.iter().rev() {
            if let Some(caps) = capture_starting_number.captures(&ws.name) {
                if let Ok(number) = &caps["number"].parse::<usize>() {
                    return (*number + 1) as u32;
                };
            }
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_workspace_fn_works() {}
}
