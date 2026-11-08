
use iced_menu::{Menu, MenuMessage, MenuConfig, MenuItem, MenuTooltip};
use iced::widget::container;
use iced::{Element, Task, Theme, Widget, window};
use std::rc::Rc;

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(Theme::TokyoNight)
        .run()
}

// User-defined enum for messages that custom menu widgets can emit
#[derive(Debug, Clone)]
enum MenuAction {
    SetDarkMode(bool),
}

// Holds all state that custom menu widgets need to reflect
#[derive(Debug, Clone, Default)]
struct State {
    is_dark_mode: bool,
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
                MenuMessage::ItemPressed(label, menubar_index) => {
                    let menubar_name = menubar_index
                        .and_then(|idx| self.menu.configs.get(idx).map(|c| c.label))
                        .unwrap_or("Unknown");
                    println!("{} > {}", menubar_name, &label);
                    self.menu.clear_timers();
                    if label == "Exit" {
                        window::latest().and_then(window::close)
                    } else {
                        Task::none()
                    }
                }
                MenuMessage::Custom(action) => match action {
                    MenuAction::SetDarkMode(checked) => {
                        self.state.is_dark_mode = checked;
                        // Rebuild configs so widgets reflect the new state
                        self.menu.configs = define_menu_configs(&self.state);
                        println!("Dark mode: {}", self.state.is_dark_mode);
                        Task::none()
                    }
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
    let size = 14.0; 
    vec![        
        MenuConfig {
            label: "File",
            items: vec![
                MenuItem::Item {
                    label: "New File", size},
                MenuItem::Item {
                    label:"New Window", size},
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "New Window with Profile >",
                        items: vec![
                            MenuItem::Item { label: "Profile 1", size },
                            MenuItem::Item { label: "Profile 2", size },
                            MenuItem::SubMenu {
                                config: MenuConfig {
                                    label: "Additional Profiles >",
                                    items: vec![
                                        MenuItem::Item {
                                            label: "Profile 3",
                                            size },
                                        MenuItem::Item {
                                            label: "Profile 4",
                                            size },
                                        MenuItem::Item {
                                            label: "Profile 5",
                                            size },
                                    ],
                                    ..Default::default()
                                },
                            },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Open File", size },
                MenuItem::Item { label: "Open Folder", size },
                MenuItem::Item { label: "Open Wokspace from File", size },
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Open Recent >",
                        items: vec![
                            MenuItem::Item {
                                label: "~/document/iced_menu", size },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Save", size },
                MenuItem::Item { label: "Save As...", size },
                MenuItem::Item { label: "Save All", size },
                MenuItem::Item { label: "Revert File", size },
                MenuItem::Item { label: "Exit", size },
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Edit",
            gap: None,
            items: vec![
                MenuItem::Item { label: "Undo", size },
                MenuItem::Item { label: "Redo", size },
                MenuItem::Item { label: "Cut", size },
                MenuItem::Item { label: "Copy", size },
                MenuItem::Item { label: "Paste", size },
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Find >",
                        items: vec![
                            MenuItem::Item { label: "Find", size },
                            MenuItem::Item { label: "Find Next", size },
                            MenuItem::Item { label: "Find Previous", size },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Item { label: "Replace", size },
                MenuItem::Item { label: "Find in Files", size },
                MenuItem::Item { label: "Replace in Files", size },
                MenuItem::Element(Rc::new(dark_mode_toggle(state.is_dark_mode))),
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Functions",
            scrollable_height: Some(300.0),
            items: functions()
                .into_iter()
                .map(|(name, signature, description)| {
                    let tooltip_label: &'static str =
                        Box::leak(format!("{signature}\n{description}").into_boxed_str());

                    MenuItem::ItemWithTooltip {
                        label: name,
                        size,
                        tooltip: MenuTooltip {
                            label: tooltip_label,
                            position: iced::widget::tooltip::Position::Right,
                        },
                    }
                })
                .collect(),
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

fn functions() -> Vec<(&'static str, &'static str, &'static str)> {
    let fs = vec![
        ("ABS", "ABS(number)", "Absolute value of a number."),
        ("ACOS", "ACOS(number)", "Arccosine in radians."),
        ("ACOSH", "ACOSH(number)", "Inverse hyperbolic cosine."),
        ("ACOT", "ACOT(number)", "Arccotangent in radians (0 to pi)."),
        ("ACOTH", "ACOTH(number)", "Inverse hyperbolic cotangent."),
        ("ASIN", "ASIN(number)", "Arcsine in radians."),
        ("ASINH", "ASINH(number)", "Inverse hyperbolic sine."),
        ("ATAN", "ATAN(number)", "Arctangent in radians."),
        ("ATAN2", "ATAN2(x_num, y_num)", "Angle of the point (x, y) from the x-axis in radians."),
        ("ATANH", "ATANH(number)", "Inverse hyperbolic tangent."),
        ];
    fs
}