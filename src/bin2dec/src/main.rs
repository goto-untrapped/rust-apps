use iced::widget::text_input;
use iced::Sandbox;

fn main() {
    // let mut settings = Settings::default();
    // settings.window.size = (360u32, 240u32);
    // let _ = ConverterBinToDec::run(settings);
    
    // can calc decimal
    // but it is hard to use iced
    // so stop in here. I will use tauri.
    // not need to use many types of gui, I'm beginner still.
    let mut converter: ConverterBinToDec = ConverterBinToDec::new(101);
    let sum = converter.converted();
    println!("{}", sum);
}

pub struct ConverterBinToDec {
    input_value: String,
    binary: u32,
    converted: u32,
}

impl ConverterBinToDec {
    pub fn new(binary_input: u32) -> ConverterBinToDec {
        ConverterBinToDec {
            input_value: String::new(),
            binary: binary_input,
            converted: 0,
        }
    }

    pub fn converted(&mut self) -> u32 {
        let binary_str = self.binary.to_string();
        let mut added: u32 = 0;
        for digit in (0..binary_str.len()).rev() {
            let digit_value = &binary_str[digit..digit + 1];
            added += digit_value.parse::<u32>().unwrap() * 2_u32.pow(digit as u32);
            println!();
        }

        added
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ConvertedPressed,
    InputChanged,
}

use iced::{widget::{button, column, text, Button}
    , Element, Alignment, alignment::Horizontal, Length};

impl Sandbox for ConverterBinToDec {
    type Message = Message;

    fn new() -> Self {
        Self {
            input_value: String::new(),
            binary: 0,
            converted: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Bin to Dec Converter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ConvertedPressed => {
                self.converted = self.converted();
            }
            Message::InputChanged => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        // We use a column: a simple vertical layout
        column![
            text(self.converted)
                .size(50).horizontal_alignment(Horizontal::Center),

            text_input("enter value", &self.input_value)
                .on_submit(Message::InputChanged)
                .size(50),

            button("Convert!").on_press(Message::ConvertedPressed),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .into()
    }
}
