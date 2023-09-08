use iced::alignment::Horizontal;
use iced::widget::{column, row, TextInput};
use iced::widget::{text_input, Button, Container, Text};
use iced::{executor, Application, Command, Element, Theme};

use iced::Settings;

struct Calculator {
    first_input: f64,
    second_input: f64,
    answer: f64,
}

#[derive(Debug, Clone)]
enum Message {
    First(String),
    Second(String),
    Third(String),
    Addition,
    Substraction,
    Division,
    Multiplication,
}

impl Application for Calculator {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flag: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                first_input: 0.0,
                second_input: 0.0,
                answer: 0.0,
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("window")
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Addition => self.answer = self.first_input + self.second_input,
            Message::Substraction => self.answer = self.first_input - self.second_input,
            Message::Multiplication => self.answer = self.first_input * self.second_input,
            Message::Division => self.answer = self.first_input / self.second_input,
            Message::First(value) => self.first_input = value.parse().unwrap_or_else(|_err|{ return 0.0; }),
            Message::Second(value) => self.second_input = value.parse().unwrap_or_else(|_err|{ return 0.0; }),
            Message::Third(value) => self.answer = value.parse().unwrap(),
        }

        Command::none()
    }
    fn view(&self) -> Element<Self::Message> {
        let input_one: Element<Message> = TextInput::new("", &self.first_input.to_string())
            .on_input(Message::First)
            .padding(10)
            .into();
        let input_two: Element<Message> = text_input("", &self.second_input.to_string())
            .padding(10)
            .on_input(Message::Second)
            .into();
        let answer_label: Element<Message> = Text::new("Answers")
            .width(400)
            .horizontal_alignment(Horizontal::Center)
            .into();
        let answer: Element<Message> = text_input("Answer", &self.answer.to_string())
            .on_input(Message::Third)
            .padding(10)
            .into();
        let add: Element<Message> = Button::new("ADD").on_press(Message::Addition).into();
        let sub: Element<Message> = Button::new("SUB").on_press(Message::Substraction).into();
        let mul: Element<Message> = Button::new("MUL").on_press(Message::Multiplication).into();
        let div: Element<Message> = Button::new("DIV").on_press(Message::Division).into();
        let btn_row: Element<Message> = row!(add, sub, mul, div).spacing(75).into();
        let answer_col: Element<Message> = column!(answer_label, answer).into();
        let main_row = column!(input_one, btn_row, input_two, answer_col)
            .padding(10)
            .spacing(40);

        Container::new(main_row).into()
    }
}

fn main() {
    let custom_settings: Settings<()> = Settings {
        window: iced::window::Settings {
            size: (400, 400),
            ..Default::default()
        },
        ..Default::default()
    };
    Calculator::run(custom_settings).unwrap()
}
