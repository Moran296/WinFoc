# WinFoc

*Focus windows with direction in X11*

this little utility let me use a keyboard shortcut to focus windows with direction
so for example SUPER + H would focus the next window to the left and SUPER + L would do the same thing for the right.

This is very handful when using more than one monior or tiled windows like environment.

## The Logic:
Once ./winfoc \<left/right/up/down\> is called it will
- search for all windows that are not hidden, minimized on the same workspace
- it will disregard any winodw that is hidden/obscured by more than 15%
- it will use the window x,y values to determine which one is the closest in the required direction
- it will focus the input and raise this window

## building
- clone
- cargo build --release
- binary is at `path/to/WinFoc/target/release/winfoc`

## Ubuntu keyboard shortcuts
- depending on the gnome version, open settings->keyboard->customize shortcuts
- add new shortcut like:
    - name: `focus left`
    - command: `/path/to/WinFoc/target/release/winfoc left` (don't use ~ or aliases, do absolute path!)
    - key: `SUPER + H`

## Plans
- support wayland (if needed)
- improve the obscured window detections mechanism
- take into account stickey windows
