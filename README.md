# SWAY Workspace manipulation helper
in progress / unstable

## Features:

- \<next\>/\<prev\> command stays on the active output
- reorder workspace by increasing/decreasing workspace Number
- swap current workspace with previous / next workspace on the currently active output
- rename workspace without changing the workspace Number
- rename/select workspace via rofi

```
> cat ~/.config/sway/config |grep swaymsg_workspace
bindsym --release Mod1+Ctrl+r exec /usr/bin/swaymsg_workspace rename_to $(rofi -dmenu -l 0 -P "rename workspace $(/usr/bin/swaymsg_workspace print_focused_name) to")
bindsym --release Mod1+Ctrl+space exec '/bin/rofi -show combi -modi combi,workspaces:/usr/bin/swaymsg_workspace'
bindsym Mod1+Ctrl+Shift+n exec /usr/bin/swaymsg_workspace swap_with_next
bindsym Mod1+Ctrl+Shift+p exec /usr/bin/swaymsg_workspace swap_with_prev
bindsym Mod1+Ctrl+f exec /usr/bin/swaymsg_workspace decrease
bindsym Mod1+Ctrl+g exec /usr/bin/swaymsg_workspace increase
bindsym Mod1+Ctrl+n exec /usr/bin/swaymsg_workspace next
bindsym Mod1+Ctrl+p exec /usr/bin/swaymsg_workspace prev
```
