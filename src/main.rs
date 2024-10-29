use std::env;
use swaymsg_workspace::execute_userinput;
pub mod ipcadapter;
pub mod workspaces;

use swaymsg_workspace::ipcadapter::{IPCAdapter, SwayIPCAdapter};
use swaymsg_workspace::workspaces::Workspaces;

fn main() {
    let ipcadapter = SwayIPCAdapter::new();
    let workspaces = Workspaces::new(ipcadapter);

    // split args by ' ' to handle the combined argument which rofi supplies / until i figured out how to read the piped in signal
    let mut args = env::args()
        .collect::<Vec<String>>()
        .into_iter()
        .flat_map(|arg| arg.split(' ').map(str::to_owned).collect::<Vec<String>>())
        .into_iter();

    if let Some(main_argument) = args.nth(1) {
        if let Err(error) = execute_userinput(
            workspaces,
            main_argument,
            args.reduce(|a, b| format!("{} {}", a, b)), // parameters to argument
        ) {
            eprintln!("Something broke: {error}");
        }
    } else {
        eprintln!("usage: swaymsg_workspace [ next prev swap_with_prev swap_with_next increase decrease rename_to select print_focused_name print_focused_number rofi_select_workspace rofi_move_window ]. ");
    }
}
