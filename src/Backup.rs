use iced::{
    button, scrollable, text_input, Align, Button, Checkbox, Color, Column, Container, Element,
    HorizontalAlignment, Image, Length, Row, Sandbox, Scrollable, Settings, Space, Text, TextInput,
    VerticalAlignment,
};

pub fn main() {
    env_logger::init();

    Tour::run(Settings::default())
}

pub struct Tour {
    steps: Steps,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
    debug: bool,
}

impl Sandbox for Tour {
    type Message = Message;

    fn new() -> Tour {
        Tour {
            steps: Steps::new(),
            scroll: scrollable::State::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
            debug: false,
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced", self.steps.title())
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Tour {
            steps,
            scroll,
            back_button,
            next_button,
            ..
        } = self;

        let mut controls = Row::new();

        if steps.has_previous() {
            controls =
                controls.push(secondary_button(back_button, "Back").on_press(Message::BackPressed));
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if steps.can_continue() {
            controls =
                controls.push(primary_button(next_button, "Next").on_press(Message::NextPressed));
        }

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(steps.view().map(Message::StepMessage))
            .push(controls)
            .into();

        let content = if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        };

        let scrollable =
            Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
                Step::TextInput {
                    value: String::new(),
                    is_secure: false,
                    state: text_input::State::new(),
                },
                Step::Calculator {
                    value: String::new(),
                    operator: String::new(),
                    any_operator: false,
                    next_value: String::new(),
                    equals_string: String::new(),
                    value_int: 0_i32,
                    one_button: button::State::new(),
                    two_button: button::State::new(),
                    three_button: button::State::new(),
                    four_button: button::State::new(),
                    five_button: button::State::new(),
                    six_button: button::State::new(),
                    seven_button: button::State::new(),
                    eight_button: button::State::new(),
                    nine_button: button::State::new(),
                    minus_button: button::State::new(),
                    zero_button: button::State::new(),
                    plus_button: button::State::new(),
                    multiply_button: button::State::new(),
                    divide_button: button::State::new(),
                    equals_button: button::State::new(),
                    clear_button: button::State::new(),
                },
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn view(&mut self) -> Element<StepMessage> {
        self.steps[self.current].view()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

enum Step {
    // see above vec
    Welcome,
    TextInput {
        value: String,
        is_secure: bool,
        state: text_input::State,
    },
    Calculator {
        value: String,
        operator: String,
        any_operator: bool,
        next_value: String,
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
        plus_button: button::State,
        multiply_button: button::State,
        divide_button: button::State,
        equals_button: button::State,
        clear_button: button::State,
    },
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    InputChanged(String),
    ToggleSecureInput(bool),
    ButtonPressed(String, String, String),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::ButtonPressed(button, left_value, right_value) => {
                if let Step::Calculator {
                    value,
                    operator,
                    next_value,
                    any_operator,
                    equals_string,
                    value_int,
                    ..
                } = self
                {
                    println!("Button Clicked {} ", button);
                    if any_operator == &false
                        && button != "+"
                        && button != "-"
                        && button != "*"
                        && button != "/"
                    {
                        println!("any_operator should be false ? {} ", any_operator);
                        *value = format!("{}{}", left_value, button);
                        println!("value {} ", value);
                    }
                    if any_operator == &true
                        && button != "+"
                        && button != "-"
                        && button != "="
                        && button != "*"
                        && button != "/"
                    {
                        println!("any_operator  is true ? {} ", any_operator);
                        *next_value = format!("{}{}", right_value, button);
                        println!("next_value {} ", next_value);
                    }
                    if button == "-" || button == "+" || button == "*" || button == "/" {
                        *any_operator = true;
                        *operator = button.to_string();
                    } else {
                        //*operator = "".to_string();
                    }
                    if button == "=" {
                        *any_operator = false;
                        *equals_string = "=".to_string();
                        println!("OPERATOR  is  ? {} ", operator);
                        println!("value  is  ? {} ", value);
                        println!("next_value  is  ? {} ", next_value);
                        let value: i32 = value.parse().unwrap();
                        let next_value: i32 = next_value.parse().unwrap();
                        if operator == "+" {
                            *value_int = value + next_value
                        } else if operator == "-" {
                            *value_int = value - next_value
                        } else if operator == "*" {
                            *value_int = value * next_value
                        } else if operator == "/" {
                            *value_int = value / next_value
                        } else {
                            *value_int = value + next_value
                        }
                    }
                    if button == "Clr" {
                        *equals_string = "".to_string();
                        *operator = "".to_string();
                        *value = "".to_string();
                        *next_value = "".to_string();
                        *value_int = 0;
                    }
                }
            }
            StepMessage::InputChanged(new_value) => {
                if let Step::TextInput { value, .. } = self {
                    *value = new_value;
                }
            }
            StepMessage::ToggleSecureInput(toggle) => {
                if let Step::TextInput { is_secure, .. } = self {
                    *is_secure = toggle;
                }
            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::TextInput { .. } => "Search Input",
            Step::Calculator { .. } => "Crypto Calculator",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::TextInput { .. } => true,
            Step::Calculator { value, .. } => value.is_empty(),
        }
    }

    fn view(&mut self) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::TextInput {
                value,
                is_secure,
                state,
            } => Self::text_input(value, *is_secure, state),
            Step::Calculator {
                value,
                operator,
                any_operator,
                next_value,
                equals_string,
                value_int,
                one_button,
                two_button,
                three_button,
                four_button,
                five_button,
                six_button,
                seven_button,
                eight_button,
                nine_button,
                minus_button,
                zero_button,
                plus_button,
                multiply_button,
                divide_button,
                equals_button,
                clear_button,
            } => Self::calculator(
                value,
                operator,
                *any_operator,
                next_value,
                equals_string,
                value_int,
                one_button,
                two_button,
                three_button,
                four_button,
                five_button,
                six_button,
                seven_button,
                eight_button,
                nine_button,
                minus_button,
                zero_button,
                plus_button,
                multiply_button,
                divide_button,
                equals_button,
                clear_button,
            ),
        }
        .into()
    }

    // this is a re-usable container
    fn container(title: &str) -> Column<'a, StepMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn welcome() -> Column<'a, StepMessage> {
        Self::container("Crypto Quick!")
            .push(Text::new(
                "Bankless Crytpo Accounting that is quick and easy to use for all.",
            ))
            .push(Image::new("resources/accounting-1.jpg"))
    }

    fn text_input(
        value: &str,
        is_secure: bool,
        state: &'a mut text_input::State,
    ) -> Column<'a, StepMessage> {
        let text_input =
            TextInput::new(state, "Type to search...", value, StepMessage::InputChanged)
                .padding(10)
                .size(30);
        Self::container("Text input")
            .push(Text::new("Search for the Crypto Market Rates."))
            .push(if is_secure {
                text_input.password()
            } else {
                text_input
            })
            .push(Checkbox::new(
                is_secure,
                "Enable password mode",
                StepMessage::ToggleSecureInput,
            ))
            .push(Text::new("See the results below:"))
            .push(
                Text::new(if value.is_empty() {
                    "You have not typed anything yet..."
                } else {
                    value
                })
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    fn calculator(
        value: &str,
        operator: &str,
        any_operator: bool,
        next_value: &str,
        equals_string: &str,
        value_int: &i32,
        one_button: &'a mut button::State,
        two_button: &'a mut button::State,
        three_button: &'a mut button::State,
        four_button: &'a mut button::State,
        five_button: &'a mut button::State,
        six_button: &'a mut button::State,
        seven_button: &'a mut button::State,
        eight_button: &'a mut button::State,
        nine_button: &'a mut button::State,
        minus_button: &'a mut button::State,
        zero_button: &'a mut button::State,
        plus_button: &'a mut button::State,
        multiply_button: &'a mut button::State,
        divide_button: &'a mut button::State,
        equals_button: &'a mut button::State,
        clear_button: &'a mut button::State,
    ) -> Column<'a, StepMessage> {
        Self::container("Crypto Quick")
            .push(Text::new("Crypto Calculator "))
            .push(Text::new(any_operator.to_string()))
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Start)
                        .spacing(25)
                        .push(Text::new(value.to_string()).size(50))
                        .push(Text::new(operator.to_string()).size(50))
                        .push(Text::new(next_value.to_string()).size(50))
                        .push(Text::new(equals_string.to_string()).size(50))
                        .push(Text::new(value_int.to_string()).size(50)),
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
                                one_button,
                                Text::new("1")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "1".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                two_button,
                                Text::new("2")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "2".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                three_button,
                                Text::new("3")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "3".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
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
                                four_button,
                                Text::new("4")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "4".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                five_button,
                                Text::new("5")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "5".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                six_button,
                                Text::new("6")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "6".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
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
                                seven_button,
                                Text::new("7")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "7".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                eight_button,
                                Text::new("8")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "8".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                nine_button,
                                Text::new("9")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "9".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
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
                                minus_button,
                                Text::new("-")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "-".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                zero_button,
                                Text::new("0")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "0".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                plus_button,
                                Text::new("+")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "+".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
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
                                multiply_button,
                                Text::new("*")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Top),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "*".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                divide_button,
                                Text::new("/")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "/".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                equals_button,
                                Text::new("=")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "=".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        )
                        .push(
                            Button::new(
                                clear_button,
                                Text::new("Clr")
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .on_press(StepMessage::ButtonPressed(
                                "Clr".to_owned(),
                                value.to_owned(),
                                next_value.to_owned(),
                            ))
                            .padding(10)
                            .min_width(60),
                        ),
                )
                .style(style_action_nav::Container),
            )
    }
}

fn button<'a, Message>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label)
            .color(Color::WHITE)
            .horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    //.border_radius(12)
    .min_width(100)
}

fn primary_button<'a, Message>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    button(state, label)
}

fn secondary_button<'a, Message>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    button(state, label)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

// This should be gracefully handled by Iced in the future. Probably using our
// own proc macro, or maybe the whole process is streamlined by `wasm-pack` at
// some point.
#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn run() {
        super::main()
    }
}

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
