use iced::{
    button, scrollable, Button, Checkbox, Color, Column, Container, Element, HorizontalAlignment,
    Length, Row, Sandbox, Scrollable, Settings, Space, Text, VerticalAlignment,
};

mod data;
//use styles;

//use styles;

//use serde::{Deserialize, Serialize};

pub fn main() {
    env_logger::init();
    Tour::run(Settings::default());
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
            steps: vec![Step::RowsAndColumns {
                is_secure: false,
                layout: Layout::Row,
                spacing: 20,
                value: String::new(),
                inbox_button: button::State::new(),
                folders_button: button::State::new(),
                tags_button: button::State::new(),
                sent_button: button::State::new(),
                spam_button: button::State::new(),
                trash_button: button::State::new(),
            }],
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
    RowsAndColumns {
        is_secure: bool,
        layout: Layout,
        spacing: u16,
        value: String,
        inbox_button: button::State,
        folders_button: button::State,
        tags_button: button::State,
        sent_button: button::State,
        spam_button: button::State,
        trash_button: button::State,
    },
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    InputChanged(String),
    ToggleSecureInput(bool),
    LayoutChanged(Layout),
    SpacingChanged(f32),
    NavButtonPressed(String),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::ToggleSecureInput(toggle) => {
                if let Step::RowsAndColumns { is_secure, .. } = self {
                    *is_secure = toggle;
                }
            }
            StepMessage::LayoutChanged(new_layout) => {
                if let Step::RowsAndColumns { layout, .. } = self {
                    *layout = new_layout;
                }
            }
            StepMessage::SpacingChanged(new_spacing) => {
                if let Step::RowsAndColumns { spacing, .. } = self {
                    *spacing = new_spacing.round() as u16;
                }
            }
            StepMessage::NavButtonPressed(button) => {
                if let Step::RowsAndColumns { value, .. } = self {
                    *value = format!("{}", button);
                    println!("value  LOAD CONTENT ? {} ", value);
                }
            }
            StepMessage::InputChanged(new_value) => {
                if let Step::RowsAndColumns { value, .. } = self {
                    *value = new_value;
                }
            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Step::RowsAndColumns { .. } => "Rows and columns",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::RowsAndColumns { .. } => true,
        }
    }

    fn view(&mut self) -> Element<StepMessage> {
        match self {
            Step::RowsAndColumns {
                is_secure,
                layout,
                spacing,
                value,
                inbox_button,
                folders_button,
                tags_button,
                sent_button,
                spam_button,
                trash_button,
            } => Self::rows_and_columns(
                *is_secure,
                *layout,
                *spacing,
                value.to_string(),
                inbox_button,
                folders_button,
                tags_button,
                sent_button,
                spam_button,
                trash_button,
            ),
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, StepMessage> {
        Column::new().spacing(20).push(Text::new(title).size(40))
    }

    fn rows_and_columns(
        is_secure: bool,
        layout: Layout,
        spacing: u16,
        value: String,
        inbox_button: &'a mut button::State,
        folders_button: &'a mut button::State,
        tags_button: &'a mut button::State,
        sent_button: &'a mut button::State,
        spam_button: &'a mut button::State,
        trash_button: &'a mut button::State,
    ) -> Column<'a, StepMessage> {
        fn new_button<'a>(state: &'a mut button::State, label: &str) -> Button<'a, StepMessage> {
            Button::new(
                state,
                Text::new(label)
                    .color(Color::BLACK)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Top),
            )
            .on_press(StepMessage::NavButtonPressed(label.to_owned()))
            .padding(3)
            .min_width(50)
            .style(data::styles::Button::Icon)
        }

        let nav_list = Column::new()
            .padding(5)
            .spacing(spacing)
            .push(new_button(inbox_button, "inbox"))
            .push(new_button(folders_button, "folders"))
            .push(new_button(tags_button, "tags"))
            .push(new_button(sent_button, "sent"))
            .push(new_button(spam_button, "spam"))
            .push(new_button(trash_button, "trash"));

        // reduced to newColumn
        fn new_column(data: Vec<&str>, is_secure: bool) -> Column<StepMessage> {
            Column::new().padding(10).spacing(5).push(data.iter().fold(
                Column::new().padding(5).spacing(10),
                |choices, language| {
                    choices.push(Checkbox::new(
                        is_secure,
                        language,
                        StepMessage::ToggleSecureInput,
                    ))
                },
            ))
        }

        let email_list = vec![
            "This is an email message you need to click and read",
            "This is an email message you need to click and read",
            "This is an email message you need to click and read",
            "This is an email message you need to click and read",
            "This is an email message you need to click and read",
            "This is an email message you need to click and read",
        ];
        let email_row = new_column(email_list, is_secure);

        let folders = vec!["my folder", "your folder", "his folder"];
        let folder_row = new_column(folders, is_secure);

        let tags = vec!["my tag", "your tag", "his tag"];
        let tag_row = new_column(tags, is_secure);

        let sents = vec!["my sent", "your sent", "his sent"];
        let sent_row = new_column(sents, is_secure);

        let spams = vec!["my spams", "your spams", "his spams"];
        let spam_row = new_column(spams, is_secure);

        let trash = vec!["my trash", "your trash", "his trash"];
        let trash_row = new_column(trash, is_secure);

        let layout_section: Element<_> = match layout {
            Layout::Row => Row::new()
                .spacing(5)
                .push(nav_list)
                .push(match value.as_str() {
                    "inbox" => email_row,
                    "folders" => folder_row,
                    "tags" => tag_row,
                    "sent" => sent_row,
                    "spam" => spam_row,
                    "trash" => trash_row,
                    _ => email_row, // default is email row
                })
                .into(),
            Layout::Column => Column::new().spacing(100).push(email_row).into(),
        };

        Self::container("Inbox View")
            .spacing(80)
            .push(layout_section)
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
