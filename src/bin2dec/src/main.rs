fn main() {
    let mut settings = Settings::default();
    settings.window.size = (360u32, 240u32);
    let _ = Counter::run(settings);
}

struct Counter {
    // The counter value
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

use iced::{widget::{button, column, text, Button, Text}
    , Sandbox, Element, Alignment, Settings, alignment::Horizontal, Length};

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
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

    fn view(&self) -> Element<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            Button::new("+").on_press(Message::IncrementPressed),

            // We show the value of the counter here
            text(self.value).size(50).horizontal_alignment(Horizontal::Center),

            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("-").on_press(Message::DecrementPressed),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .into()
    }
}