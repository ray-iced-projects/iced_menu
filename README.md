# iced_menu

The menu was built using the upcoming Iced widget Popover and Stack now under the branch popover-widget.  These widgets were extracted out and modified to run under the current Master branch.  Alignment was added to the popover to be suitable for a menu dropdown.  A CustomQuad widget, similar to the Iced example, was added for a line separator.

## Features
- ***Ease of maintaining updates***: uses two widgets maintained by Iced and no other special code.
- **Submenus**: Unlimited nesting depth with popovers opening to the right or whichever direction you chose.
- **Hover Navigation**: A delay on hover-to-open and hover-to-close for smoothness
- **Custom Widgets**: Support for custom menu items (separators, dividers, widgets, etc.)
- **Code order resembles menu layout**: The code resembles the menu layout making it easier to see how the menu is constructed.

## Usage

Define menus using the `MenuItem` enum:

```rust
MenuItem::Item { label: "Label", size: 14.0 }  // Regular menu item
MenuItem::SubMenu { config: MenuConfig { label: "Submenu >", items: vec![...], ..Default::default() } }  // Submenu with children
MenuItem::Element(Rc::new(|| widget))  // Custom widget (e.g., separator)
```
## Example - vscode_menu
This example shows a more advance menu which copied the behavior of the VsCode menu.  Only the "File" and "Edit" were completed but it will allow one to see how the construct was done which is not that more complex than the simple menu.

## Claude help
1. Implementing the timing delay and tracing the timing delay bugs (too hard for me to do). 
2. I also struggled with the more advanced menu where the Rc is used (lifetime issues).  I knew I needed Rc, but could get the code right.  After a bit of going back and forth, Claude fixed the lifetime issues.  
3. Some docs.
4. I had Claude modifiy the widgets Popover and Stack, to be used under the current Master branch and add the alignment which was trivial (since it will not be used long term, why not).  These will be replace as soon as Iced moves the widgets to master.
