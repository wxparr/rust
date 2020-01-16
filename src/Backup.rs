use iced::{
    button, Align, Application, Button, Color, Column, Command, Container, Element,
    HorizontalAlignment, Row, Settings, Text, VerticalAlignment,
};

//mod data;

pub fn main() {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    //value: i32,
    value: String,
    next_value: String,
    operator: String,
    any_operator: bool,
    equals_string: String,
    value_int: i32,
    one_button: button::State,
    two_button: button::State,
    three_button: button::State,
    four_button: button::State,
    five_button: button::State,
    six_button: button::State,
    seven_button: button::State,
    eight_button: button::State,
    nine_button: button::State,
    minus_button: button::State,
    zero_button: button::State,
    multiply_button: button::State,
    divide_button: button::State,
    plus_button: button::State,
    equals_button: button::State,
    clear_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OnePressed,
    TwoPressed,
    ThreePressed,
    FourPressed,
    FivePressed,
    SixPressed,
    SevenPressed,
    EightPressed,
    NinePressed,
    MinusPressed,
    ZeroPressed,
    PlusPressed,
    MultiplyPressed,
    DividePressed,
    EqualsPressed,
    ClearPressed,
}

#[derive(Debug)]
pub enum Background {
    Color(Color),
    Background(Color),
}

impl Application for Counter {
    type Message = Message;

    // required
    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }
    // required
    fn title(&self) -> String {
        String::from("Iced Calculator")
    }
    // required
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OnePressed => {
                if !self.any_operator {
                    self.value += concat!("1")
                } else {
                    self.next_value += concat!("1")
                };
            }
            Message::TwoPressed => {
                if !self.any_operator {
                    self.value += concat!("2")
                } else {
                    self.next_value += concat!("2")
                };
            }
            Message::ThreePressed => {
                if !self.any_operator {
                    self.value += concat!("3")
                } else {
                    self.next_value += concat!("3")
                };
            }
            Message::FourPressed => {
                if !self.any_operator {
                    self.value += concat!("4")
                } else {
                    self.next_value += concat!("4")
                };
            }
            Message::FivePressed => {
                if !self.any_operator {
                    self.value += concat!("5")
                } else {
                    self.next_value += concat!("5")
                };
            }
            Message::SixPressed => {
                if !self.any_operator {
                    self.value += concat!("6")
                } else {
                    self.next_value += concat!("6")
                };
            }
            Message::SevenPressed => {
                if !self.any_operator {
                    self.value += concat!("7")
                } else {
                    self.next_value += concat!("7")
                };
            }
            Message::EightPressed => {
                if !self.any_operator {
                    self.value += concat!("8")
                } else {
                    self.next_value += concat!("8")
                };
            }
            Message::NinePressed => {
                if !self.any_operator {
                    self.value += concat!("9")
                } else {
                    self.next_value += concat!("9")
                };
            }
            // if operator flush input value
            Message::MinusPressed => {
                self.operator = "-".to_string();
                self.any_operator = true;
            }
            Message::ZeroPressed => {
                if !self.any_operator {
                    self.value += concat!("10")
                } else {
                    self.next_value += concat!("10")
                };
            }
            Message::PlusPressed => {
                self.operator = "+".to_string();
                self.any_operator = true;
            }
            Message::MultiplyPressed => {
                self.operator = "*".to_string();
                self.any_operator = true;
            }
            Message::DividePressed => {
                self.operator = "/".to_string();
                self.any_operator = true;
            }
            Message::EqualsPressed => {
                let value: i32 = self.value.parse().unwrap();
                let next_value: i32 = self.next_value.trim().parse().unwrap();
                if self.operator == "+" {
                    self.value_int = value + next_value
                };
                if self.operator == "-" {
                    self.value_int = value - next_value
                };
                if self.operator == "*" {
                    self.value_int = value * next_value
                };
                if self.operator == "/" {
                    self.value_int = value / next_value
                };
                self.equals_string += concat!("=");
            }
            Message::ClearPressed => {
                self.any_operator = false;
                self.operator = "".to_string();
                self.value_int = 0;
                self.value = "".to_string();
                self.next_value = "".to_string();
                self.equals_string = "".to_string();
            }
        }

        Command::none()
    }
    // required
    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Start)
                        .spacing(25)
                        .push(Text::new(self.value.to_string()).size(50))
                        .push(Text::new(self.operator.to_string()).size(50))
                        .push(Text::new(self.next_value.to_string()).size(50))
                        .push(Text::new(self.equals_string.to_string()).size(50))
                        .push(Text::new(self.value_int.to_string()).size(50)),
                )
                .style(style_text_input::Container),
            )
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Start)
                        .spacing(10)
                        .push(
                            Button::new(
                                &mut self.one_button,
                                Text::new("1")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(Message::OnePressed)
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.two_button,
                                Text::new("2")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(Message::TwoPressed)
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.three_button,
                                Text::new("3")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(Message::ThreePressed)
                            .padding(10)
                            .min_width(60),
                        ),
                )
                .style(style_action_nav::Container),
            )
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Start)
                        .spacing(10)
                        .push(
                            Button::new(
                                &mut self.four_button,
                                Text::new("4")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(Message::FourPressed)
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.five_button,
                                Text::new("5")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(Message::FivePressed)
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.six_button,
                                Text::new("6")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            //.style(style_action_button::Button)
                            .on_press(Message::SixPressed)
                            .padding(10)
                            //.background
                            .min_width(60),
                        ),
                )
                .style(style_action_nav::Container),
            )
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Start)
                        .spacing(10)
                        .push(
                            Button::new(
                                &mut self.seven_button,
                                Text::new("7")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(Message::SevenPressed)
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.eight_button,
                                Text::new("8")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            //.style(style_action_button::Button)
                            .on_press(Message::EightPressed)
                            .padding(10)
                            //.background
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.nine_button,
                                Text::new("9")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            //.style(style_action_button::Button)
                            .on_press(Message::NinePressed)
                            .padding(10)
                            //.background
                            .min_width(60),
                        ),
                )
                .style(style_action_nav::Container),
            )
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Start)
                        .spacing(10)
                        .push(
                            Button::new(
                                &mut self.minus_button,
                                Text::new("-")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(Message::MinusPressed)
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.zero_button,
                                Text::new("0")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            //.style(style_action_button::Button)
                            .on_press(Message::ZeroPressed)
                            .padding(10)
                            //.background
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.plus_button,
                                Text::new("+")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            //.style(style_action_button::Button)
                            .on_press(Message::PlusPressed)
                            .padding(10)
                            //.background
                            .min_width(60),
                        ),
                )
                .style(style_action_nav::Container),
            )
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Start)
                        .spacing(10)
                        .push(
                            Button::new(
                                &mut self.multiply_button,
                                Text::new("*")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(Message::MultiplyPressed)
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.divide_button,
                                Text::new("/")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            //.style(style_action_button::Button)
                            .on_press(Message::DividePressed)
                            .padding(10)
                            //.background
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.equals_button,
                                Text::new("=")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            //.style(style_action_button::Button)
                            .on_press(Message::EqualsPressed)
                            .padding(10)
                            //.background
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                &mut self.clear_button,
                                Text::new("Clr")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            //.style(style_action_button::Button)
                            .on_press(Message::ClearPressed)
                            .padding(10)
                            //.background
                            .min_width(60),
                        ),
                )
                .style(style_action_nav::Container),
            )
            .into()
    }
}

// fn container(title: &str) -> Column<'a, StepMessage> {
//     Column::new().spacing(20).push(Text::new(title).size(50))
// }

mod style_text_input {
    use iced::{container, Background, Color};

    const BACKGROUND: Color = Color::from_rgb(0.06, 0.14, 0.21);

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(BACKGROUND)),
                border_radius: 0,
                border_color: Color::WHITE,
                border_width: 1,
                text_color: Some(Color::WHITE),
            }
        }
    }
}

mod style_action_nav {
    use iced::{container, Background, Color};

    const BACKGROUND: Color = Color::from_rgb(0.13, 0.29, 0.45);

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(BACKGROUND)),
                border_radius: 0,
                border_color: Color::WHITE,
                border_width: 2,
                text_color: Some(Color::WHITE),
            }
        }
    }
}
mod style_action_button {
    use iced::{container, Background, Color};

    const BACKGROUND: Color = Color::from_rgb(0.29, 0.00, 0.51);

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(BACKGROUND)),
                border_radius: 0,
                border_color: Color::WHITE,
                border_width: 1,
                text_color: Some(Color::WHITE),
            }
        }
    }
}
mod style_main {
    use iced::{container, Background, Color};

    const BACKGROUND: Color = Color::WHITE;

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(BACKGROUND)),
                border_radius: 0,
                border_color: Color::WHITE,
                border_width: 2,
                text_color: Some(Color::WHITE),
            }
        }
    }
}
