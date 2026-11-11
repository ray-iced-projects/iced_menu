use iced::Length::Fill;
use iced::alignment::Vertical;
use iced::widget::{self, container, space};
use iced::{Alignment, Element, Task, Theme, Widget, window};
use iced_menu::{Item, Menu, MenuConfig, MenuItem, MenuMessage};
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

        Self { menu, state }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Menu(mm) => match mm {
                MenuMessage::ItemPressed(label, bar_index) => {
                    println!(
                        "Item pressed: index {}, label{}",
                        &bar_index.unwrap(),
                        &label
                    );
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
                    }
                },
                _ => self.menu.update(mm).map(Message::Menu),
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let menu = self.menu.construct_menu().map(Message::Menu);
        container(menu).padding(10.0).boxed()
    }
}

fn define_menu_configs(state: &State) -> Vec<MenuConfig<'static, MenuAction>> {
    let file_width = 260.0;
    let menu_file_widths = (file_width * 0.4, file_width * 0.45);
    let edit_width = 200.0;
    let menu_edit_widths = (edit_width * 0.4, edit_width * 0.4);

    vec![
        MenuConfig {
            label: "File",
            width: iced::Length::Fixed(file_width),
            items: vec![
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "New Text File".into(),
                    Some("Ctrl+N".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "New File...".into(),
                    Some("Ctrl+Alt+Super+N".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "New Window".into(),
                    Some("Ctrl+Shift+N".into()),
                    None,
                ))),
                MenuItem::SubElement {
                    content: Rc::new(menu_row(
                        menu_file_widths,
                        None,
                        "New Window with Profile".into(),
                        None,
                        Some(">".to_string()),
                    )),
                    config: MenuConfig {
                        width: iced::Length::Fixed(150.0),
                        items: vec![
                            MenuItem::Item(Item::new("Profile 1")),
                            MenuItem::Item(Item::new("Profile 2")),
                            MenuItem::SubMenu {
                                config: MenuConfig {
                                    label: "Additional Profiles >",
                                    width: iced::Length::Fixed(75.0),
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
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Open File...".into(),
                    Some("Ctrl+O".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Open Folder...".into(),
                    Some("Ctrl+K Crtl+O".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Open Workspace from File...".into(),
                    None,
                    None,
                ))),
                // The sub elemnt contains both the top level item and then the children
                MenuItem::SubElement {
                    content: Rc::new(menu_row(
                        menu_file_widths,
                        None,
                        "Open Recent".into(),
                        None,
                        Some(">".into()),
                    )),
                    config: MenuConfig {
                        width: iced::Length::Fixed(240.0),
                        items: vec![MenuItem::Item(Item::new("~/document/iced_menu"))],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Save".into(),
                    Some("Ctrl+S".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Save as...".into(),
                    Some("Ctrl+Shift+S".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Save All".into(),
                    None,
                    None,
                ))),
                MenuItem::Element(Rc::new(separator())),
                // The sub elemnt contains both the top level item and then the children
                MenuItem::SubElement {
                    content: Rc::new(menu_row(
                        menu_file_widths,
                        None,
                        "Share".into(),
                        None,
                        Some(">".into()),
                    )),
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
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    Some(state.is_checked),
                    "Auto Save".into(),
                    None,
                    None,
                ))),
                // The sub elemnt contains both the top level item and then the children
                MenuItem::SubElement {
                    content: Rc::new(menu_row(
                        menu_file_widths,
                        None,
                        "Preferences".into(),
                        None,
                        Some(">".into()),
                    )),
                    config: MenuConfig {
                        width: iced::Length::Fixed(menu_file_widths.0),
                        items: vec![
                            MenuItem::Element(Rc::new(menu_row(
                                menu_file_widths,
                                None,
                                "Profiles".into(),
                                None,
                                None,
                            ))),
                            MenuItem::Element(Rc::new(menu_row(
                                menu_file_widths,
                                None,
                                "Settings".into(),
                                Some("Ctrl+,".into()),
                                None,
                            ))),
                            MenuItem::Element(Rc::new(menu_row(
                                menu_file_widths,
                                None,
                                "Extensions".into(),
                                Some("Ctrl+Shift+X,".into()),
                                None,
                            ))),
                            MenuItem::Element(Rc::new(menu_row(
                                menu_file_widths,
                                None,
                                "Keyboard Shorcuts".into(),
                                Some("Ctrl+K Ctrl+S,".into()),
                                None,
                            ))),
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Revert File".into(),
                    None,
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Close Editor".into(),
                    Some("Ctrl+W".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Close Folder".into(),
                    Some("Ctrl+k F".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Close Window".into(),
                    Some("Ctrl+F4".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(menu_row(
                    menu_file_widths,
                    None,
                    "Exit".into(),
                    Some("Ctrl+Q".into()),
                    None,
                ))),
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Edit",
            width: iced::Length::Fixed(edit_width),
            gap: None,
            items: vec![
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Undo".into(),
                    Some("Ctrl+Z".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Redo".into(),
                    Some("Ctrl+Y".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Cut".into(),
                    Some("Ctrl+X".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Copy".into(),
                    Some("Ctrl+C".into()),
                    None,
                ))),
                MenuItem::SubElement {
                    content: Rc::new(menu_row(
                        menu_edit_widths,
                        None,
                        "Copy as".into(),
                        None,
                        Some(">".into()),
                    )),
                    config: MenuConfig {
                        width: iced::Length::Fixed(250.0),
                        items: vec![
                            MenuItem::Item(Item::new("Copy Remote File URL")),
                            MenuItem::Item(Item::new("Copy Remote File URL from...")),
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Paste".into(),
                    Some("Ctrl+V".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Find".into(),
                    Some("Ctrl+F".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Replace".into(),
                    Some("Ctrl+H".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Find in Files".into(),
                    Some("Ctrl+Shift+F".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Replace in Files".into(),
                    Some("Ctrl+Shift+H".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Toggle Line Comment".into(),
                    Some("Ctrl+/".into()),
                    None,
                ))),
                MenuItem::Element(Rc::new(menu_row(
                    menu_edit_widths,
                    None,
                    "Toggle Block Comment".into(),
                    Some("Ctrl+Shift+A".into()),
                    None,
                ))),
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Selection",
            ..Default::default()
        },
        MenuConfig {
            label: "View",
            ..Default::default()
        },
        MenuConfig {
            label: "Go",
            ..Default::default()
        },
        MenuConfig {
            label: "Run",
            ..Default::default()
        },
        MenuConfig {
            label: "Terminal",
            ..Default::default()
        },
        MenuConfig {
            label: "Help",
            ..Default::default()
        },
    ]
}

fn separator() -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    || iced::widget::rule::horizontal(2.0).boxed()
}

fn menu_row(
    menu_widths: (f32, f32),
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

        let lbl = widget::text(label.clone())
            .size(size)
            .width(menu_widths.0)
            .boxed();

        let ctrl = if keys.is_some() {
            widget::text(keys.clone().unwrap())
                .size(size)
                .align_x(Alignment::End)
                .width(menu_widths.1)
                .boxed()
        } else {
            space().width(menu_widths.1).boxed()
        };

        let arr = if arrow.is_some() {
            widget::text(">").size(size).width(25.0).boxed()
        } else {
            space().width(25.0).boxed()
        };

        // Build the row with fixed total width to preserve column widths
        let rw_btn = widget::row(vec![lbl, ctrl, arr])
            .align_y(Vertical::Center)
            .width(Fill)
            .boxed();

        // Place the row into the button
        let btn_row = widget::button(rw_btn)
            .on_press(MenuMessage::Custom(MenuAction::DoAction(label.clone())))
            .padding(0.0)
            .style(move |theme, status| widget::button::text(theme, status))
            .width(Fill)
            .boxed();

        // The final row [chk/space, button with row label]
        let rw = widget::row(vec![chk, btn_row])
            .align_y(Vertical::Center)
            .width(Fill)
            .boxed();

        rw
    }
}
