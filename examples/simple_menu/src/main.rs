
use iced_menu::{Menu, MenuMessage, MenuConfig, MenuItem};
use iced::widget::container;
use iced::{Element, Task, Theme, Widget};
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
                MenuMessage::ItemPressed(label) => {
                    println!("Item pressed: {}", &label);
                    self.menu.clear_timers();
                    Task::none()
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
            width: 250.0,
            items: vec![
                MenuItem::Item {
                    label: "New File", size: 14.0},
                MenuItem::Item {
                    label:"New Window", size: 14.0},
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "New Window with Profile >",
                        width: 200.0,
                        items: vec![
                            MenuItem::Item { label: "Profile 1", size: 14.0 },
                            MenuItem::Item { label: "Profile 2", size: 14.0 },
                            MenuItem::SubMenu {
                                config: MenuConfig {
                                    label: "Additional Profiles >",
                                    width: 100.0,
                                    items: vec![
                                        MenuItem::Item {
                                            label: "Profile 3",
                                            size: 14.0 },
                                        MenuItem::Item {
                                            label: "Profile 4",
                                            size: 14.0 },
                                        MenuItem::Item {
                                            label: "Profile 5",
                                            size: 14.0 },
                                    ],
                                    ..Default::default()
                                },
                            },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Open File", size: 14.0 },
                MenuItem::Item { label: "Open Folder", size: 14.0 },
                MenuItem::Item { label: "Open Wokspace from File", size: 14.0 },
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Open Recent >",
                        width: 240.0,
                        items: vec![
                            MenuItem::Item {
                                label: "~/document/iced_menu", size: 14.0 },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item { label: "Save", size: 14.0 },
                MenuItem::Item { label: "Save As...", size: 14.0 },
                MenuItem::Item { label: "Save All", size: 14.0 },
                MenuItem::Item { label: "Revert File", size: 14.0 },
                MenuItem::Item { label: "Exit", size: 14.0 },
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
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Find >",
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
