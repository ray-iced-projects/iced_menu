
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
            width: 250.0,
            items: vec![
                MenuItem::Element(
                    Rc::new(menu_row(None, "New Text File".to_string(), Some("Ctrl+N".to_string()), None))),
                MenuItem::Element(
                    Rc::new(menu_row(None, "New File".to_string(), Some("Ctrl+Alt+Super+N".to_string()), None))),
                MenuItem::Item("New Window"),
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "New Window with Profile >",
                        width: 200.0,
                        label_size: 14.0,
                        items: vec![
                            MenuItem::Item("Profile 1"),
                            MenuItem::Item("Profile 2"),
                            MenuItem::SubMenu {
                                config: MenuConfig {
                                    label: "Additional Profiles >",
                                    width: 100.0,
                                    label_size: 14.0,
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
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item("Open File"),
                MenuItem::Item("Open Folder"),
                MenuItem::Item("Open Wokspace from File"),
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Open Recent >",
                        width: 240.0,
                        items: vec![
                            MenuItem::Item("~/document/iced_menu"),
                        ],
                        ..Default::default()
                    },
                },
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item("Save"),
                MenuItem::Item("Save As..."),
                MenuItem::Item("Save All"),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item("Share >"),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Element(Rc::new(menu_row(Some(state.is_checked), "Auto Save".to_string(), None, None))),
                MenuItem::Item("Preferences >"),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item("Revert File"),
                MenuItem::Item("Close Editor"),
                MenuItem::Item("Close Folder"),
                MenuItem::Item("Close Window"),
                MenuItem::Element(Rc::new(separator())),
                MenuItem::Item("Exit"),
            ],
            ..Default::default()
        },
        MenuConfig {
            label: "Edit",
            width: 200.0,
            gap: None,
            items: vec![
                MenuItem::Item("Undo"),
                MenuItem::Item("Redo"),
                MenuItem::Item("Cut"),
                MenuItem::Item("Copy"),
                MenuItem::Item("Paste"),
                MenuItem::SubMenu {
                    config: MenuConfig {
                        label: "Find >",
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
