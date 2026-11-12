use iced::widget::{container, column, scrollable, text, tooltip};
use iced::{Element, Padding, Task, Theme, Widget, window};
use iced_menu::{Item, Menu, MenuConfig, MenuItem, MenuMessage};
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
        Self { menu, state }
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
                },
                _ => self.menu.update(mm).map(Message::Menu),
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let menu = self.menu.construct_menu().map(Message::Menu);

        // The scroller is used to test that the opacity will block the scrolling of the menu dropdown.
        let scroll_text = 
        scrollable(column((0..50).map(|n| text(format!("scroll test {}", n)).width(200.0)))
            .spacing(2.0))
            .height(300.0)
            .boxed();

        let col = column![menu, scroll_text].boxed();

        container(col).padding(10.0).boxed()
    }
}

fn define_menu_configs(state: &State) -> Vec<MenuConfig<'static, MenuAction>> {
    vec![
        MenuConfig {
            label: "File",
            items: vec![
                MenuItem::Item(Item::new("New File")),
                MenuItem::Item(Item::new("New Window")),
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "New Window with Profile      >",
                        items: vec![
                            MenuItem::Item(Item::new("Profile 1")),
                            MenuItem::Item(Item::new("Profile 2")),
                            MenuItem::SubMenu {
                                config: MenuConfig {
                                    label: "Additional Profiles >",
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
                MenuItem::Item(Item::new("Open File")),
                MenuItem::Item(Item::new("Open Folder")),
                MenuItem::Item(Item::new("Open Wokspace from File")),
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Open Recent                            >",
                        items: vec![MenuItem::Item(Item::new("~/document/iced_menu"))],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item(Item::new("Save")),
                MenuItem::Item(Item::new("Save As...")),
                MenuItem::Item(Item::new("Save All")),
                MenuItem::Item(Item::new("Revert File")),
                MenuItem::Item(Item::new("Exit")),
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Edit",
            gap: None,
            trigger: Some(Rc::new(|| {
                container(text("Edit").size(12.0))
                    .padding(Padding::from([6, 10]))
                    .boxed()
            })),
            open_on_right_click: Some(true),
            items: vec![
                MenuItem::Item(Item::new("Undo")),
                MenuItem::Item(Item::new("Redo")),
                MenuItem::Item(Item::new("Cut")),
                MenuItem::Item(Item::new("Copy")),
                MenuItem::Item(Item::new("Paste")),
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Find                >",
                        items: vec![
                            MenuItem::Item(Item::new("Find")),
                            MenuItem::Item(Item::new("Find Next")),
                            MenuItem::Item(Item::new("Find Previous")),
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Item(Item::new("Replace")),
                MenuItem::Item(Item::new("Find in Files")),
                MenuItem::Item(Item::new("Replace in Files")),
                MenuItem::Element(Rc::new(dark_mode_toggle(state.is_dark_mode))),
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Functions",
            width: iced::Length::Fixed(65.0),
            scrollable_height: Some(150.0),
            spacing: 3.0,
            padding: Padding::from([5, 10]),
            items: functions()
                .into_iter()
                .map(|(name, signature, description)| {
                    let tooltip_label: &'static str =
                        Box::leak(format!("{signature}\n{description}").into_boxed_str());

                    MenuItem::Item(Item::new(name).tooltip(Rc::new(move |content| {
                        tooltip(
                            content,
                            container(text(tooltip_label).size(12.0))
                                .padding(8.0).style(container::rounded_box),
                        )
                        .position(iced::widget::tooltip::Position::Right)
                        .gap(10.0)
                        .boxed()
                    })))
                })
                .collect(),
            ..Default::default()
        },
    ]
}

fn separator() -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    || iced::widget::rule::horizontal(2.0).boxed()
}

fn dark_mode_toggle(checked: bool) -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    move || {
        iced::widget::checkbox(checked)
            .label("Dark Mode")
            .size(12.0)
            .text_size(12.0)
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
        (
            "ATAN2",
            "ATAN2(x_num, y_num)",
            "Angle of the point (x, y) from the x-axis in radians.",
        ),
        ("ATANH", "ATANH(number)", "Inverse hyperbolic tangent."),
    ];
    fs
}
