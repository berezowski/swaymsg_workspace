use core::panic;
use ipcadapter::IpcResult;
use std::rc::Rc;
use workspaces::extract_starting_number;
use workspaces::find_free_adjecent_workspace_num;
use workspaces::Workspace;
use workspaces::Workspaces;
pub mod ipcadapter;
pub mod workspaces;

pub enum Command {
    Next,
    Prev,
    SwapWithPrev,
    SwapWithNext,
    Increase,
    Decrease,
    RenameTo,
    Number,
    MoveContainerToWorkspaceNumber,
    Select,
    PrintFocusedName,
    PrintFocusedNumber,
    RofiSelectWorkspace,
    RofiMoveWindow,
    Usage,
}

pub fn execute_userinput(
    workspaces: Rc<Workspaces>,
    command: Command,
    argument_parameter: Option<String>,
) -> IpcResult {
    match command {
        Command::Next => match &mut workspaces
            .on_same_screen()
            .next_of(workspaces.focused_index())
        {
            Some(workspace) => workspaces.select(&workspace.basename),
            None => match argument_parameter {
                Some(_) => workspaces.select(
                    // any additional argument triggers navigation across all workspaces
                    &workspaces
                        .on_other_screen()
                        .first()
                        .or_else(|| workspaces.on_same_screen().first())
                        .unwrap()
                        .basename,
                ),
                None => workspaces.select(&workspaces.on_same_screen().first().unwrap().basename),
            },
        },
        Command::Prev => {
            match &workspaces
                .on_same_screen()
                .prev_of(workspaces.focused_index())
            {
                Some(workspace) => workspaces.select(&workspace.basename),
                None => match argument_parameter {
                    Some(_) => workspaces.select(
                        // any additional argument triggers navigation across all workspaces
                        &workspaces
                            .on_other_screen()
                            .last()
                            .or_else(|| workspaces.on_same_screen().last())
                            .unwrap()
                            .basename,
                    ),
                    None => {
                        workspaces.select(&workspaces.on_same_screen().last().unwrap().basename)
                    }
                },
            }
        }
        Command::SwapWithNext => swap_workspace(
            &workspaces,
            workspaces.on_same_screen().get(workspaces.focused_index()),
            workspaces
                .on_same_screen()
                .next_of(workspaces.focused_index()),
        ),
        Command::SwapWithPrev => swap_workspace(
            &workspaces,
            workspaces
                .on_same_screen()
                .prev_of(workspaces.focused_index()),
            workspaces.on_same_screen().get(workspaces.focused_index()),
        ),
        Command::Increase => match (
            workspaces.get_focused(),
            workspaces
                .on_same_screen()
                .next_of(workspaces.focused_index())
                .into_iter()
                // filter out anything but the workspace with the number which this increase would inhabit
                .filter(|next| &workspaces.get_focused().get_number() + 1 == next.get_number())
                .last(),
        ) {
            (ws1, Some(ws2)) => workspaces.swap(ws1, ws2),
            (ws, None) => workspaces.increase_number(ws),
        },
        Command::Decrease => {
            match (
                workspaces
                    .on_same_screen()
                    .prev_of(workspaces.focused_index())
                    .into_iter()
                    // filter out anything but the workspace with the number which this decrease would inhabit
                    .filter(|prev| prev.get_number() + 1 == workspaces.get_focused().get_number())
                    .last(),
                workspaces.get_focused(),
            ) {
                (Some(ws1), ws2) => workspaces.swap(ws1, ws2),
                (None, ws) => workspaces.decrease_number(ws),
            }
        }
        Command::RenameTo => {
            if let Some(new_name) = argument_parameter {
                workspaces.get_focused().rename(&format!(
                    "{} {}",
                    workspaces.get_focused().get_number(),
                    new_name
                ))
            } else {
                workspaces
                    .get_focused()
                    .rename(&format!("{}", workspaces.get_focused().get_number()))
            }
        }
        Command::Number => match argument_parameter {
            Some(number) => {
                if let Ok(number) = number.parse::<usize>() {
                    workspaces.select_or_create_number(number)
                } else {
                    panic!("desired workspace Number missing")
                }
            }
            _ => panic!("desired workspace Number missing"),
        },

        Command::MoveContainerToWorkspaceNumber => match argument_parameter {
            Some(number) => {
                if let Ok(number) = number.parse::<usize>() {
                    workspaces.move_container_to_number(number)
                } else {
                    panic!("desired workspace Number missing")
                }
            }
            _ => panic!("desired workspace Number missing"),
        },
        Command::Select => {
            if let Some(workspace) = argument_parameter {
                workspaces.select(&workspace)
            } else {
                panic!("Workspace missing")
            }
        }
        Command::PrintFocusedName => {
            println!("{}", workspaces.get_focused().get_name());
            // Ok(vec![workspaces.get_focused().get_name().to_string()])
            Ok(vec!["nonipc: print_focused_name".to_string()])
        }
        Command::PrintFocusedNumber => {
            println!("{}", workspaces.get_focused().get_number());
            // Ok(vec![workspaces.get_focused().get_number().to_string()])
            Ok(vec!["nonipc: print_focused_number".to_string()])
        }
        Command::RofiSelectWorkspace => match argument_parameter {
            Some(workspacename) => execute_userinput(
                workspaces.clone(),
                Command::Select,
                Some(workspacename),
            ),
            None => {
                workspaces
                    .on_all_screens()
                    .iter()
                    .for_each(|ws| println!("{}", ws.basename));
                Ok(vec!["nonipc: rofi_select_workspace".to_string()])
            }
        },
        Command::RofiMoveWindow => match argument_parameter {
            Some(workspacename) => match extract_starting_number(&workspacename) {
                Some(_number) => workspaces.move_container_to(&workspacename),
                None => workspaces.move_container_to(&format!(
                    "{} {}",
                    find_free_adjecent_workspace_num(&workspaces.on_same_screen()),
                    &workspacename
                )),
            },
            None => {
                workspaces
                    .on_all_screens()
                    .iter()
                    .for_each(|ws| println!("{}", ws.basename));
                Ok(vec!["nonipc: rofi_move_window".to_string()])
            }
        },
        Command::Usage => {
            eprintln!("valid arguments: [ next prev swap_with_prev swap_with_next increase decrease rename_to select print_focused_name print_focused_number rofi_select_workspace rofi_move_window ]. ");
            Ok(vec!["nonipc: Instructions printed".to_string()])
        }
    }.and_then(|mut dirty| { // clean workspace names from 'collision avoidance tokens'
        workspaces.cleanup().and_then( |clean| {
                dirty.extend(clean);
                Ok(dirty)
            }
        )
    })
}

fn swap_workspace(
    workspaces: &Workspaces,
    prev: Option<&Workspace>,
    next: Option<&Workspace>,
) -> IpcResult {
    match (prev, next) {
        (Some(prev), Some(next)) => workspaces.swap(prev, next),
        (Some(prev), None) => {
            swap_workspace(workspaces, Some(prev), workspaces.on_same_screen().first())
        }
        (None, Some(next)) => {
            swap_workspace(workspaces, workspaces.on_same_screen().last(), Some(next))
        }
        (None, None) => panic!("No Workspace to Swap"),
    }
}
