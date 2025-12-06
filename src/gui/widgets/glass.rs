use iced::{
    widget::{container, Container},
    Background, Border, Color, Element, Length, Shadow, Theme,
};

/// Glass container styling for glassmorphism effect
pub struct GlassContainer;

impl GlassContainer {
    /// Create a glass container with the given child
    pub fn new<'a, Message: 'a>(
        content: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message> {
        container(content)
            .padding(20)
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    1.0, 1.0, 1.0, 0.05, // Semi-transparent white
                ))),
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                },
                text_color: None,
            })
    }

    /// Glass container with hover effect
    pub fn hoverable<'a, Message: 'a>(
        content: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message> {
        container(content)
            .padding(20)
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    1.0, 1.0, 1.0, 0.08, // Slightly more opaque on hover
                ))),
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.2), // Brighter border
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                    offset: iced::Vector::new(0.0, 8.0),
                    blur_radius: 20.0,
                },
                text_color: None,
            })
    }

    /// Glass container with active/selected state
    pub fn active<'a, Message: 'a>(
        content: impl Into<Element<'a, Message>>,
    ) -> Container<'a, Message> {
        container(content)
            .padding(20)
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    0.486, 0.227, 0.929, 0.15, // Purple tint
                ))),
                border: Border {
                    color: Color::from_rgb(0.486, 0.227, 0.929), // Electric purple
                    width: 2.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.486, 0.227, 0.929, 0.4), // Purple glow
                    offset: iced::Vector::new(0.0, 0.0),
                    blur_radius: 16.0,
                },
                text_color: None,
            })
    }
}

/// Server card with glassmorphism styling
pub fn server_card<'a>(
    hostname: &'a str,
    ip: &'a str,
    tags: &'a [String],
    is_selected: bool,
    is_hovered: bool,
) -> Container<'a, crate::gui::messages::Message> {
    use iced::widget::{column, row, text, Space};

    // Hostname (bold, large)
    let hostname_text = text(hostname)
        .size(18)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.95, 0.95, 0.95)),
        });

    // IP address (dimmed, monospace)
    let ip_text = text(ip)
        .size(14)
        .font(iced::Font::MONOSPACE)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgba(0.7, 0.7, 0.7, 0.8)),
        });

    // Tags (pills)
    let mut tag_row = row![].spacing(6);
    for tag in tags {
        let tag_pill = container(
            text(tag)
                .size(12)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
                }),
        )
        .padding([4, 10])
        .style(|_theme: &Theme| container::Style {
            background: Some(Background::Color(Color::from_rgba(
                0.486, 0.227, 0.929, 0.3,
            ))),
            border: Border {
                color: Color::from_rgba(0.486, 0.227, 0.929, 0.5),
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        });
        tag_row = tag_row.push(tag_pill);
    }

    let content = column![
        hostname_text,
        ip_text,
        Space::with_height(8),
        tag_row,
    ]
    .spacing(4);

    // Apply appropriate glass style based on state
    if is_selected {
        GlassContainer::active(content)
    } else if is_hovered {
        GlassContainer::hoverable(content)
    } else {
        GlassContainer::new(content)
    }
}

/// Neon glow input styling
pub mod neon_input {
    use iced::{
        widget::{text_input, TextInput},
        Border, Color, Theme,
    };

    pub fn styled<'a, Message: Clone + 'a>(
        placeholder: &str,
        value: &str,
        on_change: impl Fn(String) -> Message + 'a,
    ) -> TextInput<'a, Message> {
        text_input(placeholder, value)
            .on_input(on_change)
            .padding(16)
            .size(18)
            .style(|_theme: &Theme, status| {
                let base_style = text_input::Style {
                    background: iced::Background::Color(Color::from_rgba(
                        0.1, 0.1, 0.15, 0.6,
                    )),
                    border: Border {
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
                        width: 1.0,
                        radius: 12.0.into(),
                    },
                    icon: Color::from_rgb(0.7, 0.7, 0.7),
                    placeholder: Color::from_rgba(0.6, 0.6, 0.6, 0.7),
                    value: Color::from_rgb(0.95, 0.95, 0.95),
                    selection: Color::from_rgba(0.486, 0.227, 0.929, 0.4),
                };

                match status {
                    text_input::Status::Focused => text_input::Style {
                        border: Border {
                            color: Color::from_rgb(0.486, 0.227, 0.929), // Purple glow
                            width: 2.0,
                            radius: 12.0.into(),
                        },
                        ..base_style
                    },
                    _ => base_style,
                }
            })
    }
}
