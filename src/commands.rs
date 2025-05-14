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
	println!("\n  Limited to current Output:");
	print_command("next", "Switch to next Workspace on current Output");
	print_command("prev", "Switch to previous Workspace on current Output");
	print_command(
		"swap_with_prev",
		"Swap current Workspace with next Workspace on current Output",
	);
	print_command(
		"swap_with_next",
		"Swap current Workspace with previous Workspace on current Output",
	);
	print_command("increase", "Increase Indexnumber of current Workspace");
	print_command("decrease", "Decrease Indexnumber of current Workspace");
	print_command("rename_to ARGUMENT", "Rename current Workspace to ARGUMENT");
	print_command(
		"number ARGUMENT",
		"Select Workspace Indexed ARUGMENT on current Output",
	);
	print_command(
		"move_container_to_workspace_number ARGUMENT",
		"Move Container to Workspace Indexed ARGUMENT on current Output",
	);
	print_command(
		"print_focused_name",
		"Print Current Workspace Name without Indexnumber",
	);
	print_command(
		"print_focused_number",
		"Print Current Workspace Indexnumber",
	);
	println!("\n  Across any Outputs:");
	print_command(
		"select ARGUMENT",
		"Select Workspace named ARUGMENT on any Output",
	);
	print_command(
		"rofi_select_workspace ARGUMENT",
		"alias for 'select ARGUMENT'",
	);
	print_command(
		"rofi_move_window ARGUMENT",
		"Move Container to Workspace by full Name ARGUMENT on any Output",
	);
	println!("\n");
}

fn print_command(command: &str, description: &str) {
	eprintln!("  {command:<45}{description}");
}
