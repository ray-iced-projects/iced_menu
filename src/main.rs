

use iced::widget::{button, text, column, container, row};
use iced::{Element, Task, Theme, Widget};

mod widgets;
use crate::widgets::popover::{Popover, Position};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
    .theme(Theme::TokyoNight)
    .run()
}

struct App<'a> {
    is_open: Vec<bool>,
    dropdown_labels: DropdownLabels<'a>,
}

#[derive(Debug, Clone)]
enum Message {
    Close(usize),
    File(String),
}

impl <'a>App<'a> {
    fn default() -> Self {
        Self {
            is_open: vec![false; 8],
            dropdown_labels: DropdownLabels::default(),
        }
    }
}

impl <'a>App<'a> {
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close(idx) => {
                self.is_open[idx] = !self.is_open[idx];
                Task::none()
            },
            Message::File(label) => {
                dbg!(&label);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        
        let mb = get_menu_bar(&self);
        
        container(mb)
        .padding(10.0)
        .boxed()

    }
}


fn get_menu_bar<'a>(app: &'a App) -> Element<'a, Message> {

    let labels = ["File", "Edit", "Selection", "View", "Go", "Run", "Terminal", "Help"];

    let mut menu_bar = vec![];
    
    for (idx, label) in labels.iter().enumerate() {
        let pop = match *label {
            "File" => get_dropdown(app, idx, app.dropdown_labels.file.clone()),
            "Edit" => get_dropdown(app, idx, app.dropdown_labels.edit.clone()),
            _ => None
        };
        menu_bar.push(Popover::new(
            button(text(*label))
                    .on_press(Message::Close(idx))
                    .style(move|theme, status|
                            button::text(theme, status)
                        )   
                    .boxed(),
            pop,
        )
        .gap(5.0)
        .position(Position::Bottom)
        .on_close(Message::Close(idx)).boxed());
    }
    
    row(menu_bar).boxed()

}


struct DropdownLabels<'a> {
    file: Vec<&'a str>,
    edit: Vec<&'a str>,
}

impl <'a>DropdownLabels<'a> {
    fn default() -> Self {
        Self {
            file: vec![
                "New File", 
                "New Window",
                "Open Folder",
                "Open Recent",
                "Save",
                "Save As...",
                "Save All",
                "Revert File",
                "Exit",
            ],
            edit: vec![
                "Undo",
                "Redo",
                "Cut",
                "Copy",
                "Paste",
                "Find",
                "Replace",
                "Find in Files",
                "Replace in Files",
        ]
        }
    }
}
fn get_dropdown<'a>(app: &App, idx: usize, labels: Vec<&'a str>) 
    -> Option<Element<'a, Message>> {

    let mut btns = vec![];
    
    for label in labels {
        btns.push(button(label)
            .on_press(Message::File(label.to_string()))
            .style(move|theme, status| 
                button::text(theme, status))
            .boxed())
    }

    let col = 
        column(btns).boxed();
    
    let content = if app.is_open[idx] {
            Some(
                container(col)
                    .width(200.0)
                    .height(200.0)
                    .padding(0)
                    .style(move|theme| container::bordered_box(theme))
                    .boxed()
            )
        } else {
            None
        };

    content
}
