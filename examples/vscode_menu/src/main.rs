
use iced::{Length, Alignment, Element, Task, Theme, Widget, window};
use iced::alignment::Vertical;
use iced_menu::{Menu, MenuMessage, MenuConfig, MenuItem, Item};
use iced::widget::{self, container};
use std::rc::Rc;


pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(Theme::TokyoNight)
        .window_size((700, 500))
        .centered()
        .run()
}

// User-defined enum for messages that custom menu widgets can emit
#[derive(Debug, Clone)]
enum MenuAction {
    DoAction(String),
    SetAutoSave(bool),
}

// Holds all state that custom menu widgets need
#[derive(Debug, Clone, Default)]
struct State {
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
                MenuMessage::ItemPressed(label, bar_index) => {
                    println!("Item pressed: index {}, label{}", &bar_index.unwrap(), &label);
                    self.menu.clear_timers();
                    Task::none()
                }
                MenuMessage::Custom(action) => match action {
                    MenuAction::DoAction(label) => {
                        println!("Action Item pressed: {}", label);
                        if label == "Exit" {
                            window::latest().and_then(window::close)
                        } else {
                            Task::none()
                        }
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
        let menu = self.menu.construct_menu().map(Message::Menu);
        container(menu)
            .padding(10.0)
            .boxed()
    }
}

fn define_menu_configs(state: &State) -> Vec<MenuConfig<'static, MenuAction>> {
    
    vec![        
        MenuConfig {
            label: "File",
            width: iced::Length::Fixed(315.0),
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
                        width: iced::Length::Fixed(200.0),
                        items: vec![
                            MenuItem::Item(Item::new("Profile 1")),
                            MenuItem::Item(Item::new("Profile 2")),
                            MenuItem::SubMenu {
                                config: MenuConfig {
                                    label: "Additional Profiles >",
                                    width: iced::Length::Fixed(100.0),
                                    items: vec![
                                        MenuItem::Item(Item::new("Profile 3")),
                                        MenuItem::Item(Item::new("Profile 4")),
                                        MenuItem::Item(Item::new("Profile 5")),
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
                        width: iced::Length::Fixed(240.0),
                        items: vec![
                            MenuItem::Item(Item::new("~/document/iced_menu")),
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
                        width: iced::Length::Fixed(240.0),
                        items: vec![
                            MenuItem::Item(Item::new("Copy vscode.dec Link")),
                            MenuItem::Element(Rc::new(separator())),
                            MenuItem::Item(Item::new("Export Profile (default)")),
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
                        width: iced::Length::Fixed(300.0),
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
            width: iced::Length::Fixed(350.0),
            gap: None,
            items: vec![
                MenuItem::Element(Rc::new(
                    menu_row(None, "Undo".to_string(), Some("Ctrl+Z".to_string()), None ))),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Redo".to_string(), Some("Ctrl+Y".to_string()), None ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Cut".to_string(), Some("Ctrl+X".to_string()), None ))),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Copy".to_string(), Some("Ctrl+C".to_string()), None ))),
                MenuItem::SubElement {
                    content: Rc::new(sub_menu_row("Copy as")),
                    config: MenuConfig {
                        width: iced::Length::Fixed(250.0),
                        items: vec![
                            MenuItem::Item(Item::new("Copy Remote File URL")),
                            MenuItem::Item(Item::new("Copy Remote File URL from...")),
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(
                    menu_row(None, "Paste".to_string(), Some("Ctrl+V".to_string()), None ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Find".to_string(), Some("Ctrl+F".to_string()), None ))),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Replace".to_string(), Some("Ctrl+H".to_string()), None ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Find in Files".to_string(), Some("Ctrl+Shift+F".to_string()), None ))),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Replace in Files".to_string(), Some("Ctrl+Shift+H".to_string()), None ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Toggle Line Comment".to_string(), Some("Ctrl+/".to_string()), None ))),
                MenuItem::Element(Rc::new(
                    menu_row(None, "Toggle Block Comment".to_string(), Some("Ctrl+Shift+A".to_string()), None ))),
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Selection",
            width: iced::Length::Fixed(0.0),
            ..Default::default()
        },
        MenuConfig {
            label: "View",
            width: iced::Length::Fixed(0.0),
            ..Default::default()
        },
        MenuConfig {
            label: "Go",
            width: iced::Length::Fixed(0.0),
            ..Default::default()
        },
        MenuConfig {
            label: "Run",
            width: iced::Length::Fixed(0.0),
            ..Default::default()
        },
        MenuConfig {
            label: "Terminal",
            width: iced::Length::Fixed(0.0),
            ..Default::default()
        },
        MenuConfig {
            label: "Help",
            width: iced::Length::Fixed(0.0),
            ..Default::default()
        },
    ]
}

fn separator() -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    || {
        iced::widget::rule::horizontal(2.0).boxed()
    }
}

fn menu_row(
    checked: Option<bool>, 
    label: String, 
    keys: Option<String>, 
    arrow: Option<String>,
) -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    let size = 12.0;
    move || {
        let chk = if checked.is_some() {
            widget::checkbox(checked.unwrap())
            .on_toggle(|b| MenuMessage::Custom(MenuAction::SetAutoSave(b)))
            .spacing(0.0)
            .label(" ")
            .boxed()
        } else {
            widget::space().width(20.0).boxed()
        };

        let lbl = 
            widget::text(label.clone())
            .size(size)
            .width(150.0)
            .boxed();

        let ctrl = 
            if keys.is_some() {
            widget::text(keys.clone().unwrap())
            .size(size)
            .align_x(Alignment::End)
            .width(115.0)
            .boxed()
        } else {
            widget::text("          ").boxed()
        };

    let arr = 
        if arrow.is_some() {
            widget::text(">")
            .size(size)
            .width(25.0)
            .boxed()
        } else {
            widget::space().width(25.0).boxed()
        };
    
    // Build the row with fixed total width to preserve column widths
    let rw_btn = 
        widget::row(vec![lbl, ctrl, arr])
        .align_y(Vertical::Center)
        .width(Length::Fixed(290.0))
        .boxed();

    // Place the row into the button
    let btn_row = 
        widget::button(rw_btn)
            .on_press(MenuMessage::Custom(MenuAction::DoAction(label.clone())))
            .padding(0.0)
            .style(move|theme, status| 
                widget::button::text(theme, status))
            .width(Length::Fill)
            .boxed();
    // The final row [chk/space, button with row label]
    let rw = 
        widget::row(vec![chk, btn_row])
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .boxed();

    rw
    }

}

/// Returns button-interior content for use with MenuItem::SubElement.
/// Matches the visual layout of menu_row: spacer | label (fill) | ">" arrow.
fn sub_menu_row(label: &'static str) -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    move || {
        let size = 12.0;
        let spc = widget::space().width(20.0).boxed();
        let lbl = widget::text(label).size(size).width(Length::Fill).boxed();
        let arr = widget::text(">").size(size).align_x(Alignment::End).width(25.0).boxed();
        widget::row(vec![spc, lbl, arr])
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .boxed()
    }
}
