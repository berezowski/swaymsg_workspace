# SWAY Workspace manipulation helper
in progress / unstable

#### motivation:
i was unhappy how sway handles user interaction with workspaces especially when you have additional monitors connected.
This program tries very hard to keep all workspace operations on the output where the the currently focused container resides.
Also it seconds as a replacement for a bash script to integrate workspaces in rofi.


## Features:

- \<next\>/\<prev\> command stays on the active output
- reorder workspace by increasing/decreasing workspace Number
- swap current workspace with previous / next workspace on the currently active output
- rename workspace without changing the workspace Number
- rename/select workspace via rofi
- move container to workspace on same output
- select workspace by number on same output

```
> cat ~/.config/sway/config |grep swaymsg_workspace

bindsym Mod1+Ctrl+Shift+n exec /usr/bin/swaymsg_workspace swap_with_next
bindsym Mod1+Ctrl+Shift+p exec /usr/bin/swaymsg_workspace swap_with_prev
bindsym Mod1+Ctrl+f exec /usr/bin/swaymsg_workspace decrease
bindsym Mod1+Ctrl+g exec /usr/bin/swaymsg_workspace increase
bindsym Mod1+Ctrl+n exec /usr/bin/swaymsg_workspace next
bindsym Mod1+Ctrl+p exec /usr/bin/swaymsg_workspace prev

bindsym --release Mod1+Ctrl+r exec /usr/bin/swaymsg_workspace rename_to $(rofi -dmenu -l 0 -P "rename workspace $(/usr/bin/swaymsg_workspace print_focused_name) to")

bindsym Mod1+Ctrl+0 exec /usr/bin/swaymsg_workspace number 10
bindsym Mod1+Ctrl+1 exec /usr/bin/swaymsg_workspace number 1
bindsym Mod1+Ctrl+2 exec /usr/bin/swaymsg_workspace number 2
bindsym Mod1+Ctrl+3 exec /usr/bin/swaymsg_workspace number 3
bindsym Mod1+Ctrl+4 exec /usr/bin/swaymsg_workspace number 4
bindsym Mod1+Ctrl+5 exec /usr/bin/swaymsg_workspace number 5
bindsym Mod1+Ctrl+6 exec /usr/bin/swaymsg_workspace number 6
bindsym Mod1+Ctrl+7 exec /usr/bin/swaymsg_workspace number 7
bindsym Mod1+Ctrl+8 exec /usr/bin/swaymsg_workspace number 8
bindsym Mod1+Ctrl+9 exec /usr/bin/swaymsg_workspace number 9
bindsym Mod1+Ctrl+Shift+0 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 10
bindsym Mod1+Ctrl+Shift+1 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 1
bindsym Mod1+Ctrl+Shift+2 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 2
bindsym Mod1+Ctrl+Shift+3 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 3
bindsym Mod1+Ctrl+Shift+4 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 4
bindsym Mod1+Ctrl+Shift+5 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 5
bindsym Mod1+Ctrl+Shift+6 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 6
bindsym Mod1+Ctrl+Shift+7 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 7
bindsym Mod1+Ctrl+Shift+8 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 8
bindsym Mod1+Ctrl+Shift+9 exec /usr/bin/swaymsg_workspace move_container_to_workspace_number 9

```

```
> cat ~/.config/rofi/config.rasi
configuration {
  modi: "combi,move to workspace:/usr/bin/swaymsg_workspace rofi_move_window";
  font: "M+CodeLat60 Nerd Font Mono 12";
  combi-modi: "workspaces:/usr/bin/swaymsg_workspace rofi_select_workspace,window,drun,ssh";
  kb-mode-next: "Control+Alt+space";
}

```
