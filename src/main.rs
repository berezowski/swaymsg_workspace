use std::env;

use workspaces::Workspace;
use workspaces::Workspaces;
pub mod workspaces;

// type WS<'a> = WorkspaceDetails<'a>;

fn main() {
    // split args by ' ' to handle the combined argument which rofi supplies / until i figured out how to read the piped in signal
    // std::process::Command::status()
    // dbg!(std::process::ExitStatus);
    let mut args = env::args()
        .collect::<Vec<String>>()
        .into_iter()
        .flat_map(|arg| arg.split(' ').map(str::to_owned).collect::<Vec<String>>())
        .into_iter();

    if let Some(main_argument) = args.nth(1) {
        execute_userinput(
            main_argument,
            args.reduce(|a, b| format!("{} {}", a, b)), // parameters to argument
        );
    } else {
        eprintln!("usage: swaymsg_workspace [ next prev swap_with_prev swap_with_next increase decrease rename_to select print_focused_name print_focused_number rofi_select_workspace rofi_move_window ]. ");
    }
}

fn execute_userinput(argument: String, argument_parameter: Option<String>) {
    let workspaces = Workspaces::new();
    match argument.as_str() {
        "next" => match &mut workspaces
            .on_same_screen
            .next_of(workspaces.focused_index())
        {
            Some(workspace) => {
                workspaces.select(&workspace.basename);
            }
            None => {
                match argument_parameter {
                    Some(_) => {
                        workspaces.select(
                            // any additional argument triggers navigation across all workspaces
                            &workspaces
                                .on_other_screen
                                .first()
                                .or_else(|| workspaces.on_same_screen.first())
                                .unwrap()
                                .basename,
                        );
                    }
                    None => workspaces.select(&workspaces.on_same_screen.first().unwrap().basename),
                }
            }
        },
        "prev" => {
            match &workspaces
                .on_same_screen
                .prev_of(workspaces.focused_index())
            {
                Some(workspace) => workspaces.select(&workspace.basename),
                None => match argument_parameter {
                    Some(_) => workspaces.select(
                        // any additional argument triggers navigation across all workspaces
                        &workspaces
                            .on_other_screen
                            .last()
                            .or_else(|| workspaces.on_same_screen.last())
                            .unwrap()
                            .basename,
                    ),
                    None => workspaces.select(&workspaces.on_same_screen.last().unwrap().basename),
                },
            }
        }
        "swap_with_next" => {
            swap_workspace(
                &workspaces,
                workspaces.on_same_screen.get(workspaces.focused_index()),
                workspaces
                    .on_same_screen
                    .next_of(workspaces.focused_index()),
            );
        }
        "swap_with_prev" => {
            if workspaces.focused_index() < 1 {
                swap_workspace(
                    &workspaces,
                    None,
                    workspaces.on_same_screen.get(workspaces.focused_index()),
                );
            } else {
                swap_workspace(
                    &workspaces,
                    workspaces
                        .on_same_screen
                        .get(workspaces.focused_index() - 1),
                    workspaces.on_same_screen.get(workspaces.focused_index()),
                );
            }
        }
        "increase" => match (
            workspaces.get_focused(),
            workspaces
                .on_same_screen
                .get(workspaces.focused_index() + 1)
                .into_iter()
                .filter(|next| &workspaces.get_focused().get_number() + 1 == next.get_number())
                .last(),
        ) {
            (ws1, Some(ws2)) => workspaces.swap(ws1, ws2),
            (ws, None) => workspaces.increase_index(ws),
        },
        "decrease" => {
            if workspaces.focused_index() > 0 {
                match (
                    workspaces
                        .on_same_screen
                        .get(workspaces.focused_index() - 1)
                        .into_iter()
                        .filter(|prev| {
                            prev.get_number() + 1 == workspaces.get_focused().get_number()
                        })
                        .last(),
                    workspaces.get_focused(),
                ) {
                    (Some(ws1), ws2) => {
                        workspaces.swap(ws1, ws2);
                    }
                    (None, ws) => workspaces.decrease_index(ws),
                }
            } else {
                workspaces.decrease_index(workspaces.get_focused());
            }
        }
        "rename_to" => {
            let _ = if let Some(new_name) = argument_parameter {
                workspaces.get_focused().rename(
                    format!("{} {}", workspaces.get_focused().get_number(), new_name).as_str(),
                );
            } else {
                workspaces
                    .get_focused()
                    .rename(format!("{}", workspaces.get_focused().get_number()).as_str());
            };
        }
        "select" => {
            let _ = if let Some(workspace) = argument_parameter {
                workspaces.select(&workspace);
            };
        }
        "print_focused_name" => {
            println!("{}", workspaces.get_focused().get_name());
        }
        "print_focused_number" => {
            println!("{}", workspaces.get_focused().get_number());
        }
        "rofi_select_workspace" => match argument_parameter {
            Some(workspacename) => execute_userinput("select".to_string(), Some(workspacename)),
            None => workspaces
                // .on_all_screens()
                .on_same_screen
                .iter()
                .chain(workspaces.on_other_screen.iter())
                .for_each(|ws| println!("{}", ws.basename)),
        },
        "rofi_move_window" => match argument_parameter {
            Some(workspacename) => workspaces.move_window(&workspacename),
            None => workspaces
                // .on_all_screens()
                .on_same_screen
                .iter()
                .chain(workspaces.on_other_screen.iter())
                .for_each(|ws| println!("{}", ws.basename)),
        },
        _ => {
            eprintln!("valid arguments: [ next prev swap_with_prev swap_with_next increase decrease rename_to select print_focused_name print_focused_number rofi_select_workspace rofi_move_window ]. ");
        }
    }
    workspaces.cleanup();
}

fn swap_workspace(wss: &Workspaces, prev: Option<&Workspace>, next: Option<&Workspace>) {
    match (prev, next) {
        (Some(prev), Some(next)) => wss.swap(prev, next),
        (Some(prev), None) => swap_workspace(wss, Some(prev), wss.on_same_screen.first()),
        (None, Some(next)) => swap_workspace(wss, wss.on_same_screen.last(), Some(next)),
        (None, None) => {}
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn rename_workspace_fn_works() {}
// }
