
use iced_menu::{Menu, MenuMessage, MenuConfig, MenuItem};
use iced::widget::{container, space};
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

struct App {
    menu: Menu<'static, MenuAction>,
    is_dark_mode: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Menu(MenuMessage<MenuAction>),
}

impl App {
    fn default() -> Self {
        let is_dark_mode = false;
        let configs = define_menu_configs(is_dark_mode);
        let len = configs.len();
        let mut menu = Menu::default();
        menu.configs = configs;
        menu.is_open = vec![false; len];
        Self {
            menu,
            is_dark_mode,
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
                        self.is_dark_mode = checked;
                        // Rebuild configs so the checkbox reflects the new state
                        self.menu.configs = define_menu_configs(self.is_dark_mode);
                        println!("Dark mode: {}", self.is_dark_mode);
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

fn define_menu_configs(is_dark_mode: bool) -> Vec<MenuConfig<'static, MenuAction>> {
    vec![        
        MenuConfig {
            bar_label: "File",
            width: 250.0,
            items: vec![
                MenuItem::Item("New File"),
                MenuItem::Item("New Window"),
                MenuItem::SubMenu {
                    label: "New Window with Profile >",
                    config: MenuConfig {
                        width: 200.0,
                        items: vec![
                            MenuItem::Item("Profile 1"),
                            MenuItem::Item("Profile 2"),
                            MenuItem::SubMenu {
                                label: "Additional Profiles >",
                                config: MenuConfig {
                                    width: 100.0,
                                    items: vec![
                                        MenuItem::Item("Profile 3"),
                                        MenuItem::Item("Profile 4"),
                                        MenuItem::Item("Profile 5"),
                                    ],
                                    ..Default::default()
                                },
                            },
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Custom(Rc::new(separator())),
                MenuItem::Item("Open File"),
                MenuItem::Item("Open Folder"),
                MenuItem::Item("Open Wokspace from File"),
                MenuItem::SubMenu {
                    label: "Open Recent >",
                    config: MenuConfig {
                        width: 240.0,
                        items: vec![
                            MenuItem::Item("~/document/iced_menu"),
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Custom(Rc::new(separator())),
                MenuItem::Item("Save"),
                MenuItem::Item("Save As..."),
                MenuItem::Item("Save All"),
                MenuItem::Item("Revert File"),
                MenuItem::Item("Exit"),
            ],
            ..Default::default()
        },
        MenuConfig {
            bar_label: "Edit",
            width: 200.0,
            gap: None,
            items: vec![
                MenuItem::Item("Undo"),
                MenuItem::Item("Redo"),
                MenuItem::Item("Cut"),
                MenuItem::Item("Copy"),
                MenuItem::Item("Paste"),
                MenuItem::SubMenu {
                    label: "Find >",
                    config: MenuConfig {
                        width: 150.0,
                        items: vec![
                            MenuItem::Item("Find"),
                            MenuItem::Item("Find Next"),
                            MenuItem::Item("Find Previous"),
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Item("Replace"),
                MenuItem::Item("Find in Files"),
                MenuItem::Item("Replace in Files"),
                MenuItem::Custom(Rc::new(dark_mode_toggle(is_dark_mode))),
            ],
            ..Default::default()
        }
    ]
}

fn separator() -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    || {
        space()
            .height(iced::Length::Fixed(1.0))
            .width(iced::Length::Fill)
            .boxed()
    }
}

// Real checkbox widget — captures `checked` at config-build time;
// configs are rebuilt in update() after each toggle so state stays current
fn dark_mode_toggle(checked: bool) -> impl Fn() -> Element<'static, MenuMessage<MenuAction>> {
    move || {
        iced::widget::checkbox(checked)
            .label("Dark Mode")
            .on_toggle(|b| MenuMessage::Custom(MenuAction::SetDarkMode(b)))
            .boxed()
    }
}
