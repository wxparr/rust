use iced::{
    button, Align, Application, Button, Color, Column, Command, Container, Element,
    HorizontalAlignment, Image, Row, Settings, Text, VerticalAlignment,
};

mod data;

//use data::ipfs;

pub fn main() {
    // ipfs::new();

    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    primary_button: button::State,
    secondary_button: button::State,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
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
        String::from("Fuzzy Net")
    }
    // required
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
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
                        .spacing(100)
                        //.push(Text::new(title).size(50))
                        .push(Image::new("resources/fuzzynet.png"))
                        .push(
                            Button::new(
                                &mut self.primary_button,
                                Text::new("Add File")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(Message::IncrementPressed)
                            .padding(30)
                            .min_width(100),
                        )
                        .push(Text::new(self.value.to_string()).size(50))
                        .push(
                            Button::new(
                                &mut self.secondary_button,
                                Text::new("Delete File")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(Message::DecrementPressed)
                            .padding(30)
                            .min_width(100),
                        ),
                )
                .style(style_nav::Container),
            )
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Start)
                        .spacing(100)
                        //.push(Text::new(title).size(50))
                        .push(
                            Button::new(
                                &mut self.increment_button,
                                Text::new("Add File")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(Message::IncrementPressed)
                            .padding(30)
                            .min_width(100),
                        )
                        .push(Text::new(self.value.to_string()).size(50))
                        .push(
                            Button::new(
                                &mut self.decrement_button,
                                Text::new("Delete File")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            //.style(style_action_button::Button)
                            .on_press(Message::DecrementPressed)
                            .padding(30)
                            //.background
                            .min_width(100),
                        ),
                )
                .style(style_action_nav::Container),
            )
            .push(
                Container::new(
                    Column::new()
                        .align_items(Align::Center)
                        .spacing(150)
                        //.background(Color::BLACK)
                        //.push(Text::new(title).size(50))
                        .push(
                            Text::new(
                                "It was originally born as part of Coffee, an opinionated \
                         2D game engine for Rust.",
                            )
                            .color(Color::BLACK),
                        )
                        .push(
                            Text::new("some middle content."), // .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        //Image::new("resources/fuzzynet_large.png")
                        .push(Image::new("resources/fuzzynet_large.png"))
                        .push(
                            Text::new("LSTE HAVE SOME MAIN CONTENT."), //.horizontal_alignment(HorizontalAlignment::Center),
                        ),
                )
                .style(style_main::Container),
            )
            .into()
    }
}

// fn container(title: &str) -> Column<'a, StepMessage> {
//     Column::new().spacing(20).push(Text::new(title).size(50))
// }

mod style_nav {
    use iced::{container, Background, Color};

    const BACKGROUND: Color = Color::from_rgb(0.76, 0.20, 0.98);

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
                border_width: 1,
                text_color: Some(Color::WHITE),
            }
        }
    }
}
