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

pub fn print_usage() {
	println!("Usage: swaymsg_workspace COMMAD [ARGUMENTS]\n");
	println!("Commands:");
	println!("\n  Limited to current output:");
	print_command("next", "Switch to next workspace on current output");
	print_command("prev", "Switch to previous workspace on current output");
	print_command(
		"swap_with_prev",
		"Swap current workspace with next workspace on current output",
	);
	print_command(
		"swap_with_next",
		"Swap current workspace with previous workspace on current output",
	);
	print_command("increase", "Increase indexnumber of current workspace");
	print_command("decrease", "Decrease indexnumber of current workspace");
	print_command("rename_to ARGUMENT", "Rename current workspace to ARGUMENT");
	print_command(
		"number ARGUMENT",
		"Select workspace indexed aRUGMENT on current output",
	);
	print_command(
		"move_container_to_workspace_number ARGUMENT",
		"Move container to workspace indexed ARGUMENT on current output",
	);
	print_command(
		"print_focused_name",
		"Print current workspace name without indexnumber",
	);
	print_command(
		"print_focused_number",
		"Print current workspace indexnumber",
	);
	println!("\n  Across any outputs:");
	print_command(
		"rofi_select_workspace",
		"print unique names of all workspaces",
	);
	print_command(
		"rofi_select_workspace ARGUMENT",
		"Select workspace by unique name ARGUMENT on any output",
	);
	print_command(
		"select ARGUMENT",
		"alias for 'rofi_select_workspace ARGUMENT'",
	);
	print_command("rofi_move_window", "print unique names of all workspaces");
	print_command(
		"rofi_move_window ARGUMENT",
		"Move container to workspace by unique name ARGUMENT on any output",
	);
	println!("\n");
}

fn print_command(command: &str, description: &str) {
	eprintln!("  {command:<45}{description}");
}
