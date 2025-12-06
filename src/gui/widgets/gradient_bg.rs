use iced::{widget::container, Color, Element, Length, Background, Border, Gradient};

pub struct GradientBackground;

impl GradientBackground {
    pub fn view<'a, Message: 'a>() -> Element<'a, Message> {
        container("")
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Gradient(Gradient::Linear(
                    iced::gradient::Linear::new(0.0)
                        .add_stop(0.0, Color::from_rgb(0.059, 0.090, 0.165)) // Deep navy
                        .add_stop(0.3, Color::from_rgb(0.08, 0.12, 0.22))    // Slightly lighter
                        .add_stop(0.6, Color::from_rgb(0.12, 0.08, 0.25))    // Purple tint
                        .add_stop(1.0, Color::from_rgb(0.15, 0.05, 0.20))    // Deep purple
                ))),
                ..Default::default()
            })
            .into()
    }
}
