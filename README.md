# iced_menu

This menu repo is not a library yet, just a proof of concept for a menu lib in the future.  The menu was built using the upcoming Iced widget Popover and Stack now under the branch popover-widget.  These widgets were extracted out and modified to run under the current Master branch.  Alignment was added to the popover to be suitable for a menu dropdown.  A CustomQuad widget, similar to the Iced example, was added for a line separator.

## Features
- ***Ease of maintaining updates***: uses two widgets maintained by Iced and no other special code.
- **Submenus**: Unlimited nesting depth with popovers opening to the right or whichever direction you chose.
- **Hover Navigation**: A delay on hover-to-open and hover-to-close for smoothness
- **Custom Widgets**: Support for custom menu items (separators, dividers, widgets, etc.)

## Usage

Define menus using the `MenuItem` enum:

```rust
MenuItem::Item("Label")  // Regular menu item
MenuItem::SubMenu { label: "Submenu >", items: vec![...] }  // Submenu with children
MenuItem::Custom { key: "line" }  // Custom widget (e.g., separator)
```

## Claude help
Implementing the timing delay and tracing the timing delay bugs (too hard for me to do). I have a similar widget in the IcedPyGui repo that I'm updating so most of the work was done except for no timing yet.  My PopUp I was using had some bugs so when I saw the popover widget in the iced branch, I had Claude modifiy it to use under the current Master branch and add the alignment which was trivial.
