

use iced::widget::{button, text, column, container, text_input};
use iced::{Element, Fill, Task, Theme, Widget};

mod widgets;
use crate::widgets::popover::{Popover, Position};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
    .theme(Theme::TokyoNight)
    .run()
}

#[derive(Default)]
struct App {
    is_open: bool,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    Close,
    InputChanged(String),
    OnSubmit,
}

impl App {
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close => {
                self.is_open = !self.is_open;
                Task::none()
            }
            Message::InputChanged(value) => {
                self.input_value = value;
                Task::none()
            }
            Message::OnSubmit => {
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // The base is always present. The `popover` argument is `Some` when the
        // popover is open and `None` when it is closed.
        let input: Element<Message> = text_input("Some placeholder", &self.input_value)
            .width(150.0)
            .on_input(Message::InputChanged)
            .on_submit(Message::OnSubmit)
            .boxed();
            
        let popover_content: Option<Element<Message>> = if self.is_open {
            Some(
                container("This is the popover contents!")
                    .width(200.0)
                    .height(200.0)
                    .padding(0)
                    .style(move|theme| container::primary(theme))
                    .boxed()
            )
        } else {
            None
        };
        
        let pop = Popover::new(
            button(text("Click me!")).on_press(Message::Close).boxed(),
            popover_content,
        )
        .gap(5.0)
        .position(Position::Top)
        .on_close(Message::Close).boxed();
        
        container(
            column(vec![
                input,
                pop,
            ])
        ).center(Fill)
        .boxed()
    }
}
