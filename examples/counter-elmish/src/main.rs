mod detail;
use crate::detail::{OnEvent, App, Settings};
use livid::{enums::*, prelude::*, *};

pub fn main() {
    Counter::new().run(Settings {
        size: (300, 200),
        win_color: Some(Color::Custom((250, 250, 250))),
        ..Default::default()
    })
}

#[derive(Default, Copy, Clone)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl App for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - livid")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) {
        let col = group::Column::default_fill();
        button::Button::default()
            .with_label("Increment")
            .on_trigger(Message::IncrementPressed);
        frame::Frame::default().with_label(&self.value.to_string());
        button::Button::default()
            .with_label("Decrement")
            .on_trigger(Message::DecrementPressed);
        col.end();
    }
}
