use iced::{widget::{button, column, container, pick_list, row, text, Space}, Element, Length, Color, Background, Border};
use crate::gui::messages::Message;
use crate::gui::state::NebulaVaultState;
use crate::terminal_launcher::TerminalApp;

pub fn view_settings(state: &NebulaVaultState) -> Element<'_, Message> {
    let title = text("Settings")
        .size(32)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.95, 0.95, 0.95)),
        });

    let subtitle = text("Configure your Nebula Vault experience")
        .size(14)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgba(0.7, 0.7, 0.7, 0.9)),
        });

    // Terminal preference section with glass styling
    let section_title = text("Terminal Preference")
        .size(18)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
        });

    let terminal_label = text("Preferred Terminal Application:")
        .size(14)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgba(0.8, 0.8, 0.8, 0.9)),
        });

    // Get available terminals
    let available_terminals = TerminalApp::detect_available();
    
    let terminal_picker = pick_list(
        available_terminals,
        Some(&state.terminal_preference),
        Message::TerminalPreferenceChanged,
    )
    .placeholder("Select terminal...")
    .width(Length::Fixed(300.0))
    .style(|theme, status| {
        let base = pick_list::Style {
            background: Background::Color(Color::from_rgba(0.1, 0.1, 0.15, 0.6)),
            border: Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Color::from_rgb(0.95, 0.95, 0.95),
            placeholder_color: Color::from_rgba(0.6, 0.6, 0.6, 0.7),
            handle_color: Color::from_rgba(0.486, 0.227, 0.929, 0.8),
        };
        
        match status {
            pick_list::Status::Active => pick_list::Style {
                border: Border {
                    color: Color::from_rgba(0.486, 0.227, 0.929, 0.6),
                    width: 2.0,
                    radius: 8.0.into(),
                },
                ..base
            },
            _ => base,
        }
    });

    let current_terminal = text(format!(
        "Currently using: {}",
        state.terminal_preference.display_name()
    ))
    .size(13)
    .style(|_theme| text::Style {
        color: Some(Color::from_rgba(0.486, 0.227, 0.929, 0.9)),
    });

    // Glass container for terminal settings
    let terminal_section = container(
        column![
            section_title,
            Space::with_height(12),
            terminal_label,
            terminal_picker,
            Space::with_height(8),
            current_terminal,
        ]
        .spacing(8)
    )
    .padding(24)
    .width(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    });

    // Back button with glass styling
    let back_button = button(
        text("← Back")
            .size(16)
    )
    .on_press(Message::CloseSettings)
    .padding([12, 24])
    .style(|_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgba(0.486, 0.227, 0.929, 0.3),
            _ => Color::from_rgba(0.486, 0.227, 0.929, 0.2),
        })),
        border: Border {
            color: Color::from_rgba(0.486, 0.227, 0.929, 0.5),
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Color::from_rgb(0.95, 0.95, 0.95),
        ..Default::default()
    });

    let content = column![
        title,
        subtitle,
        Space::with_height(32),
        terminal_section,
        Space::with_height(24),
        back_button,
    ]
    .spacing(8)
    .padding(40)
    .width(Length::Fill)
    .max_width(700);

    // Center the content
    let centered = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .padding([60, 0]);

    // Deep space background
    container(centered)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.059, 0.090, 0.165))),
            ..Default::default()
        })
        .into()
}
