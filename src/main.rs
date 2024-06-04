use std::env;

use workspaces::Workspace;
use workspaces::Workspaces;
pub mod workspaces;

// type WS<'a> = WorkspaceDetails<'a>;

fn main() {
    // split args by ' ' to handle the combined argument which rofi supplies / until i figured out how to read the piped in signal
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
    let wss = Workspaces::new();
    match argument.as_str() {
        "next" => match &mut wss.same_screen_workspaces.get(wss.focused_index() + 1) {
            Some(workspace) => {
                wss.select(&workspace.basename);
            }
            None => {
                match argument_parameter {
                    Some(_) => {
                        wss.select(
                            // any additional argument triggers navigation across all workspaces
                            &wss.other_screen_workspaces
                                .first()
                                .or_else(|| wss.same_screen_workspaces.first())
                                .unwrap()
                                .basename,
                        );
                    }
                    None => wss.select(&wss.same_screen_workspaces.first().unwrap().basename),
                }
                // &wss.active_workspaces.first().unwrap().basename);
            }
        },
        "prev" => {
            if *&wss.focused_index() > 0 {
                wss.select(
                    &wss.same_screen_workspaces
                        .get(wss.focused_index() - 1)
                        .unwrap()
                        .basename,
                );
            } else {
                match argument_parameter {
                    Some(_) => wss.select(
                        // any additional argument triggers navigation across all workspaces
                        &wss.other_screen_workspaces
                            .last()
                            .or_else(|| wss.same_screen_workspaces.last())
                            .unwrap()
                            .basename,
                    ),
                    None => wss.select(&wss.same_screen_workspaces.last().unwrap().basename),
                }
            }
        }
        "swap_with_next" => {
            swap_workspace(
                &wss,
                wss.same_screen_workspaces.get(wss.focused_index()),
                wss.same_screen_workspaces.get(wss.focused_index() + 1),
            );
        }
        "swap_with_prev" => {
            if wss.focused_index() < 1 {
                swap_workspace(
                    &wss,
                    None,
                    wss.same_screen_workspaces.get(wss.focused_index()),
                );
            } else {
                swap_workspace(
                    &wss,
                    wss.same_screen_workspaces.get(wss.focused_index() - 1),
                    wss.same_screen_workspaces.get(wss.focused_index()),
                );
            }
        }
        "increase" => match (
            wss.focused(),
            wss.same_screen_workspaces
                .get(wss.focused_index() + 1)
                .into_iter()
                .filter(|next| &wss.focused().get_number() + 1 == next.get_number())
                .last(),
        ) {
            (ws1, Some(ws2)) => wss.swap(ws1, ws2),
            (ws, None) => wss.increase_index(ws),
        },
        "decrease" => {
            if wss.focused_index() > 0 {
                match (
                    wss.same_screen_workspaces
                        .get(wss.focused_index() - 1)
                        .into_iter()
                        .filter(|prev| prev.get_number() + 1 == wss.focused().get_number())
                        .last(),
                    wss.focused(),
                ) {
                    (Some(ws1), ws2) => {
                        wss.swap(ws1, ws2);
                    }
                    (None, ws) => wss.decrease_index(ws),
                }
            } else {
                wss.decrease_index(wss.focused());
            }
        }
        "rename_to" => {
            let _ = if let Some(new_name) = argument_parameter {
                wss.focused()
                    .rename(format!("{} {}", wss.focused().get_number(), new_name).as_str());
            } else {
                wss.focused()
                    .rename(format!("{}", wss.focused().get_number()).as_str());
            };
        }
        "select" => {
            let _ = if let Some(workspace) = argument_parameter {
                wss.select(&workspace);
            };
        }
        "print_focused_name" => {
            println!("{}", wss.focused().get_name());
        }
        "print_focused_number" => {
            println!("{}", wss.focused().get_number());
        }
        "rofi_select_workspace" => match argument_parameter {
            Some(workspacename) => execute_userinput("select".to_string(), Some(workspacename)),
            None => wss
                .same_screen_workspaces
                .iter()
                .chain(wss.other_screen_workspaces.iter())
                .for_each(|ws| println!("{}", ws.basename)),
        },
        "rofi_move_window" => match argument_parameter {
            Some(workspacename) => wss.move_window(&workspacename),
            None => wss
                .same_screen_workspaces
                .iter()
                .chain(wss.other_screen_workspaces.iter())
                .for_each(|ws| println!("{}", ws.basename)),
        },
        _ => {
            eprintln!("valid arguments: [ next prev swap_with_prev swap_with_next increase decrease rename_to select print_focused_name print_focused_number rofi_select_workspace rofi_move_window ]. ");
        }
    }
    wss.cleanup();
}

fn swap_workspace(wss: &Workspaces, prev: Option<&Workspace>, next: Option<&Workspace>) {
    match (prev, next) {
        (Some(prev), Some(next)) => wss.swap(prev, next),
        (Some(prev), None) => swap_workspace(wss, Some(prev), wss.same_screen_workspaces.first()),
        (None, Some(next)) => swap_workspace(wss, wss.same_screen_workspaces.last(), Some(next)),
        (None, None) => {}
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn rename_workspace_fn_works() {}
// }
