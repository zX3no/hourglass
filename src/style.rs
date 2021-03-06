use iced::{button, Background, Color, Vector};

pub enum Button {
    Primary,
    Hover,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                Button::Hover => Color::from_rgb(0.8, 0.2, 0.2),
            })),
            border_radius: 6.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }
}
