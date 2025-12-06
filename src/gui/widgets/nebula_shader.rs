use iced::{widget::container, Color, Element, Length, Background, Border};
use std::time::Instant;

pub struct NebulaShader {
    _start_time: Instant,
}

impl NebulaShader {
    pub fn new() -> Self {
        Self {
            _start_time: Instant::now(),
        }
    }

    pub fn view<'a, Message: 'a>(&'a self) -> Element<'a, Message> {
        // Simplified nebula background using gradients
        // TODO: Replace with actual WGSL shader when canvas support is available
        container("")
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.059, 0.090, 0.165))),
                ..Default::default()
            })
            .into()
    }
}

impl Default for NebulaShader {
    fn default() -> Self {
        Self::new()
    }
}
