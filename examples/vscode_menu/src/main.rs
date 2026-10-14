
use iced::Length::Fill;
use iced::alignment::Vertical;
use iced_menu::{Menu, MenuMessage, MenuConfig, MenuItem};
use iced::widget::{self, container};
use iced::{Alignment, Element, Task, Theme, Widget};
use std::rc::Rc;

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(Theme::TokyoNight)
        .run()
}

// User-defined enum for messages that custom menu widgets can emit
#[derive(Debug, Clone)]
enum MenuAction {
    DoAction,
    SetDarkMode(bool),
    SetAutoSave(bool),
}

// Holds all state that custom menu widgets need to reflect
#[derive(Debug, Clone, Default)]
struct State {
    is_dark_mode: bool,
    is_checked: bool,
}

struct App {
    menu: Menu<'static, MenuAction>,
    state: State,
}

#[derive(Debug, Clone)]
enum Message {
    Menu(MenuMessage<MenuAction>),
}

impl App {
    fn default() -> Self {
        let state = State::default();
        let configs = define_menu_configs(&state);
        let len = configs.len();
        let mut menu = Menu::default();
        menu.configs = configs;
        menu.is_open = vec![false; len];
        Self {
            menu,
            state,
        }
    }
}

impl App {
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Menu(mm) => match mm {
                MenuMessage::ItemPressed(label) => {
                    println!("Item pressed: {}", &label);
                    self.menu.clear_timers();
                    Task::none()
                }
                MenuMessage::Custom(action) => match action {
                    MenuAction::DoAction => {
                        println!("Doing some action");
                        Task::none()
                    }
                    MenuAction::SetDarkMode(checked) => {
                        self.state.is_dark_mode = checked;
                        // Rebuild configs so widgets reflect the new state
                        self.menu.configs = define_menu_configs(&self.state);
                        println!("Dark mode: {}", self.state.is_dark_mode);
                        Task::none()
                    }
                    MenuAction::SetAutoSave(is_checked) => {
                        println!("SetAutoSave: {}", is_checked);
                        self.state.is_checked = is_checked;
                        self.menu.configs = define_menu_configs(&self.state);
                        Task::none()
                    },
                }
                _ => self.menu.update(mm).map(Message::Menu)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let menu = self.menu.get_menu_static().map(Message::Menu);
        container(menu)
            .padding(10.0)
            .boxed()
    }
}

fn define_menu_configs(state: &State) -> Vec<MenuConfig<'static, MenuAction>> {
    vec![        
        MenuConfig {
            label: "File",
            label_size: 14.0,
            width: 300.0,
            items: vec![
                MenuItem::Element(
                    Rc::new(
                        menu_row(None, "New Text File".to_string(), Some("Ctrl+N".to_string()), None))),
                MenuItem::Element(
                    Rc::new(menu_row(
                        None, "New File...".to_string(), Some("Ctrl+Alt+Super+N".to_string()), None))),
                MenuItem::Element(
                    Rc::new(menu_row(
                        None, "New Window".to_string(), Some("Ctrl+Shift+N".to_string()), None ))),
                MenuItem::SubElement {
                    content: Rc::new(sub_menu_row("New Window with Profile")),
                    config: MenuConfig {
                        width: 200.0,
                        items: vec![
                            MenuItem::Item { label: "Profile 1", size: 14.0 },
                            MenuItem::Item { label: "Profile 2", size: 14.0 },
                            MenuItem::SubMenu {
                                config: MenuConfig {
                                    label: "Additional Profiles >",
                                    width: 100.0,
                                    label_size: 14.0,
                                    items: vec![
                                        MenuItem::Item { label: "Profile 3", size: 14.0 },
                                        MenuItem::Item { label: "Profile 4", size: 14.0 },
                                        MenuItem::Item { label: "Profile 5", size: 14.0 },
                                    ],
                                    ..Default::default()
                                },
                            },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(
                    Rc::new(menu_row(
                        None, "Open File...".to_string(), Some("Ctrl+O".to_string()), None ))),
                MenuItem::Element(
                    Rc::new(menu_row(
                        None, "Open Folder...".to_string(), Some("Ctrl+K Crtl+O".to_string()), None ))),
                MenuItem::Element(
                    Rc::new(menu_row(
                        None, "Open Workspace from File...".to_string(), None, None ))),
                // The sub elemnt contains both the top level item and then the children
                MenuItem::SubElement {
                    content: Rc::new(sub_menu_row("Open Recent")),
                    config: MenuConfig {
                        width: 240.0,
                        items: vec![
                            MenuItem::Item { label: "~/document/iced_menu", size: 14.0 },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(
                    Rc::new(menu_row(
                        None, "Save".to_string(), Some("Ctrl+S".to_string()), None ))),
                MenuItem::Element(
                    Rc::new(menu_row(
                        None, "Save as...".to_string(), Some("Ctrl+Shift+S".to_string()), None ))),
                MenuItem::Element(
                    Rc::new(menu_row(
                        None, "Save All".to_string(), None, None ))),
                MenuItem::Element(Rc::new(separator())),
                // The sub elemnt contains both the top level item and then the children
                MenuItem::SubElement {
                    content: Rc::new(sub_menu_row("Share")),
                    config: MenuConfig {
                        width: 240.0,
                        items: vec![
                            MenuItem::Item { label: "Copy vscode.dec Link", size: 14.0 },
                            MenuItem::Element(Rc::new(separator())),
                            MenuItem::Item { label: "Export Profile (default)", size: 14.0 },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(menu_row(Some(state.is_checked), "Auto Save".to_string(), None, None))),
                // The sub elemnt contains both the top level item and then the children
                MenuItem::SubElement {
                    content: Rc::new(sub_menu_row("Preferences")),
                    config: MenuConfig {
                        width: 350.0,
                        items: vec![
                            MenuItem::Element(Rc::new(
                                menu_row(None, "Profiles".to_string(), None, None ))),
                            MenuItem::Element(Rc::new(
                                menu_row(None, "Settings".to_string(), Some("Ctrl+,".to_string()), None ))),
                            MenuItem::Element(Rc::new(
                                menu_row(None, "Extensions".to_string(), Some("Ctrl+Shift+X,".to_string()), None ))),
                            MenuItem::Element(Rc::new(
                                menu_row(None, "Keyboard Shorcuts".to_string(), Some("Ctrl+K Ctrl+S,".to_string()), None ))),
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Revert File".to_string(), None, None ))),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Close Editor".to_string(), Some("Ctrl+W".to_string()), None ))),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Close Folder".to_string(), Some("Ctrl+k F".to_string()), None ))),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Close Window".to_string(), Some("Ctrl+F4".to_string()), None ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Exit".to_string(), Some("Ctrl+Q".to_string()), None ))),
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Edit",
            width: 200.0,
            gap: None,
            items: vec![
                MenuItem::Item { label: "Undo", size: 14.0 },
                MenuItem::Item { label: "Redo", size: 14.0 },
                MenuItem::Item { label: "Cut", size: 14.0 },
                MenuItem::Item { label: "Copy", size: 14.0 },
                MenuItem::Item { label: "Paste", size: 14.0 },
                MenuItem::SubElement {
                    content: Rc::new(sub_menu_row("Find >")),
                    config: MenuConfig {
                        width: 150.0,
                        items: vec![
                            MenuItem::Item { label: "Find", size: 14.0 },
                            MenuItem::Item { label: "Find Next", size: 14.0 },
                            MenuItem::Item { label: "Find Previous", size: 14.0 },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Item { label: "Replace", size: 14.0 },
                MenuItem::Item { label: "Find in Files", size: 14.0 },
                MenuItem::Item { label: "Replace in Files", size: 14.0 },
                MenuItem::Element(Rc::new(dark_mode_toggle(state.is_dark_mode))),
            ],
            ..Default::default()
        }
    ]
}

fn separator() -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    || {
        iced::widget::rule::horizontal(2.0).boxed()
    }
}


fn dark_mode_toggle(checked: bool) -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    move || {
        iced::widget::checkbox(checked)
            .label("Dark Mode")
            .on_toggle(|b| MenuMessage::Custom(MenuAction::SetDarkMode(b)))
            .boxed()
    }
}

fn menu_row(
    checked: Option<bool>, 
    label: String, 
    keys: Option<String>, 
    arrow: Option<String>,
) -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    let size = 14.0;
    move || {
        let chk = if checked.is_some() {
            widget::checkbox(checked.unwrap())
            .on_toggle(|b| MenuMessage::Custom(MenuAction::SetAutoSave(b)))
            .width(20.0)
            .boxed()
        } else {
            widget::space().width(20.0).boxed()
        };

        let lbl = 
            widget::text(label.clone())
            .size(size)
            .width(Fill)
            .boxed();

        let ctrl = 
            if keys.is_some() {
            widget::text(keys.clone().unwrap())
            .size(size)
            .align_x(Alignment::End)
            .width(Fill)
            .boxed()
        } else {
            widget::space().boxed()
        };

    let arr = 
        if arrow.is_some() {
            widget::text(">")
            .size(size)
            .align_x(Alignment::End)
            .width(25.0)
            .boxed()
        } else {
            widget::space().width(25.0).boxed()
        };

    let rw_btn = 
        widget::row(vec![lbl, ctrl, arr])
        .align_y(Vertical::Center)
        .width(Fill)
        .boxed();

    let btn = 
        widget::button(rw_btn)
            .on_press(MenuMessage::Custom(MenuAction::DoAction))
            .padding(0.0)
            .style(move|theme, status| 
                widget::button::text(theme, status))
            .width(Fill)
            .boxed();

    let rw = 
        widget::row(vec![chk, btn])
        .spacing(10.0)
        .align_y(Vertical::Center)
        .width(Fill)
        .boxed();

    rw
    }

}

/// Returns button-interior content for use with MenuItem::SubElement.
/// Matches the visual layout of menu_row: spacer | label (fill) | ">" arrow.
fn sub_menu_row(label: &'static str) -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    move || {
        let size = 14.0;
        let spc = widget::space().width(20.0).boxed();
        let lbl = widget::text(label).size(size).width(Fill).boxed();
        let arr = widget::text(">").size(size).align_x(Alignment::End).width(25.0).boxed();
        widget::row(vec![spc, lbl, arr])
            .align_y(Vertical::Center)
            .width(Fill)
            .boxed()
    }
}
