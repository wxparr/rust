use iced::{button, Background, Color, Font, HorizontalAlignment, Length, Text, Vector};
use serde::{Deserialize, Serialize};
// Fonts - Icons
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../resources/fuzzynet.png"),
};

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

// pub fn edit_icon() -> Text {
//     icon('\u{F303}')
// }

// fn delete_icon() -> Text {
//     icon('\u{F1F8}')
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

pub enum Button {
    Filter { selected: bool },
    Icon,
    Destructive,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Filter { selected } => {
                if *selected {
                    button::Style {
                        background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
                        border_radius: 10,
                        text_color: Color::WHITE,
                        ..button::Style::default()
                    }
                } else {
                    button::Style::default()
                }
            }
            Button::Icon => button::Style {
                text_color: Color::from_rgb(0.5, 0.5, 0.5),
                ..button::Style::default()
            },
            Button::Destructive => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                border_radius: 5,
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 1.0),
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            text_color: match self {
                Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
                Button::Filter { selected } if !selected => Color::from_rgb(0.2, 0.2, 0.7),
                _ => active.text_color,
            },
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }
}
