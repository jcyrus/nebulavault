use iced::{widget::{button, column, container, row, scrollable, text, Column, Stack, Image, Space}, Element, Length, Color, Background, Border, Gradient};
use crate::gui::messages::Message;
use crate::gui::state::NebulaVaultState;
use crate::gui::widgets::GradientBackground;

pub fn view_main(state: &NebulaVaultState) -> Element<'_, Message> {
    let sidebar = render_sidebar(state);
    let main_content = render_main_content(state);
    let content_area = row![sidebar, main_content].spacing(0);

    // Layer UI over gradient background
    let ui_layer = container(content_area)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: None,
            ..Default::default()
        });

    // Gradient background
    let background = GradientBackground::view();

    // Outer container
    container(
        Stack::new()
            .push(background)
            .push(ui_layer)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn render_sidebar(state: &NebulaVaultState) -> Element<'_, Message> {
    let title_row = row![
        text("Connections")
            .size(20)
            .style(|_theme| text::Style {
                color: Some(iced::Color::from_rgb(0.9, 0.9, 0.9)),
            }),
        button(text("+").size(20))
            .on_press(Message::ShowAddHostDialog)
            .padding([4, 12])
            .style(|_theme, status| button::Style {
                background: Some(Background::Color(match status {
                    button::Status::Hovered => Color::from_rgba(0.486, 0.227, 0.929, 0.4),
                    _ => Color::from_rgba(0.486, 0.227, 0.929, 0.3),
                })),
                border: Border {
                    color: Color::from_rgba(0.486, 0.227, 0.929, 0.6),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                text_color: Color::WHITE,
                ..Default::default()
            }),
    ]
    .spacing(10)
    .align_y(iced::Alignment::Center);

    let mut host_list = Column::new().spacing(8).padding(16);

    if state.hosts.is_empty() {
        let empty_text = text("No connections yet")
            .size(14)
            .style(|_theme| text::Style {
                color: Some(iced::Color::from_rgb(0.6, 0.6, 0.65)),
            });
        host_list = host_list.push(empty_text);
    } else {
        for host in &state.hosts {
            host_list = host_list.push(render_host_item(&host.id, &host.name, &host.hostname));
        }
    }

    let sidebar_content = column![title_row, scrollable(host_list)]
        .spacing(16)
        .padding(16);

    container(sidebar_content)
        .width(Length::Fixed(280.0))
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.03))),
            border: Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(2.0, 0.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        })
        .into()
}

fn render_host_item(id: &str, name: &str, hostname: &str) -> Element<'static, Message> {
    let id_owned = id.to_string();
    let id_for_edit = id.to_string();
    let id_for_delete = id.to_string();
    let name_owned = name.to_string();
    let hostname_owned = hostname.to_string();

    let name_text = text(name_owned)
        .size(14)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(0.95, 0.95, 0.95)),
        });

    let hostname_text = text(hostname_owned)
        .size(12)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(0.6, 0.6, 0.65)),
        });

    let info_column = column![name_text, hostname_text]
        .spacing(4)
        .width(Length::Fill);

    // Action buttons with text labels instead of icons
    let edit_button = button(text("Edit").size(12))
        .on_press(Message::ShowEditHostDialog(id_for_edit))
        .padding([4, 8])
        .style(|_theme, status| button::Style {
            background: Some(iced::Background::Color(match status {
                button::Status::Hovered => iced::Color::from_rgb(0.3, 0.5, 0.7),
                _ => iced::Color::from_rgb(0.25, 0.25, 0.28),
            })),
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            text_color: iced::Color::WHITE,
            ..Default::default()
        });

    let delete_button = button(text("Del").size(12))
        .on_press(Message::ShowDeleteConfirm(id_for_delete))
        .padding([4, 8])
        .style(|_theme, status| button::Style {
            background: Some(iced::Background::Color(match status {
                button::Status::Hovered => iced::Color::from_rgb(0.8, 0.3, 0.3),
                _ => iced::Color::from_rgb(0.25, 0.25, 0.28),
            })),
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            text_color: iced::Color::WHITE,
            ..Default::default()
        });

    let actions = row![edit_button, delete_button].spacing(4);

    // Main clickable area
    let item_row = row![info_column, actions]
        .spacing(8)
        .align_y(iced::Alignment::Center)
        .padding(12)
        .width(Length::Fill);

    button(item_row)
        .on_press(Message::Connect(id_owned))
        .width(Length::Fill)
        .style(|_theme, status| button::Style {
            background: Some(Background::Gradient(Gradient::Linear(
                iced::gradient::Linear::new(135.0) // Diagonal glassy gradient
                    .add_stop(0.0, match status {
                        button::Status::Hovered => Color::from_rgba(0.4, 0.3, 0.8, 0.25),
                        _ => Color::from_rgba(0.3, 0.2, 0.6, 0.15),
                    })
                    .add_stop(1.0, match status {
                        button::Status::Hovered => Color::from_rgba(0.3, 0.5, 0.9, 0.2),
                        _ => Color::from_rgba(0.2, 0.4, 0.7, 0.1),
                    })
            ))),
            border: Border {
                color: match status {
                    button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.3),
                    _ => Color::from_rgba(1.0, 1.0, 1.0, 0.12),
                },
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: iced::Shadow {
                color: match status {
                    button::Status::Hovered => Color::from_rgba(0.4, 0.3, 0.8, 0.4),
                    _ => Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                },
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: match status {
                    button::Status::Hovered => 12.0,
                    _ => 4.0,
                },
            },
            ..Default::default()
        })
        .into()
}

fn render_main_content(state: &NebulaVaultState) -> Element<'_, Message> {
    use iced::widget::Space;
    
    // Welcome header
    let welcome = text("Nebula Vault")
        .size(48)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgb(0.95, 0.95, 0.95)),
        });
    
    let subtitle = text("SSH Connection Manager")
        .size(18)
        .style(|_theme| text::Style {
            color: Some(Color::from_rgba(0.7, 0.7, 0.7, 0.9)),
        });

    // Status with glass background
    let status_content = text(format!("{} connection{} configured", 
        state.hosts.len(),
        if state.hosts.len() == 1 { "" } else { "s" }
    ))
    .size(14)
    .style(|_theme| text::Style {
        color: Some(Color::from_rgba(0.8, 0.8, 0.8, 0.9)),
    });

    let status_card = container(status_content)
        .padding(16)
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
            border: Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        });

    // Action buttons in glass cards with gradients
    let manage_identities_button = button(
        container(
            column![
                Image::new("assets/icons/keys.png")
                    .width(48)
                    .height(48),
                Space::with_height(4),
                text("Configure SSH keys & passwords")
                    .size(12)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.9)),
                    }),
            ]
            .align_x(iced::Alignment::Center)
        )
        .padding(24)
        .width(Length::Fixed(240.0))
    )
    .on_press(Message::ShowIdentityList)
    .style(|_theme, status| button::Style {
        background: Some(Background::Gradient(Gradient::Linear(
            iced::gradient::Linear::new(135.0) // Diagonal gradient
                .add_stop(0.0, match status {
                    button::Status::Hovered => Color::from_rgb(0.2, 0.7, 0.6), // Bright teal
                    _ => Color::from_rgb(0.15, 0.55, 0.5), // Darker teal
                })
                .add_stop(1.0, match status {
                    button::Status::Hovered => Color::from_rgb(0.3, 0.8, 0.4), // Bright green
                    _ => Color::from_rgb(0.2, 0.65, 0.35), // Darker green
                })
        ))),
        border: Border {
            color: match status {
                button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.3),
                _ => Color::from_rgba(1.0, 1.0, 1.0, 0.15),
            },
            width: match status {
                button::Status::Hovered => 2.0,
                _ => 1.0,
            },
            radius: 16.0.into(),
        },
        shadow: iced::Shadow {
            color: match status {
                button::Status::Hovered => Color::from_rgba(0.2, 0.7, 0.5, 0.5),
                _ => Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            },
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: match status {
                button::Status::Hovered => 24.0,
                _ => 12.0,
            },
        },
        ..Default::default()
    });

    let settings_button = button(
        container(
            column![
                Image::new("assets/icons/settings.png")
                    .width(48)
                    .height(48),
                Space::with_height(4),
                text("Configure terminal preferences")
                    .size(12)
                    .style(|_theme| text::Style {
                        color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.9)),
                    }),
            ]
            .align_x(iced::Alignment::Center)
        )
        .padding(24)
        .width(Length::Fixed(240.0))
    )
    .on_press(Message::ShowSettings)
    .style(|_theme, status| button::Style {
        background: Some(Background::Gradient(Gradient::Linear(
            iced::gradient::Linear::new(135.0) // Diagonal gradient
                .add_stop(0.0, match status {
                    button::Status::Hovered => Color::from_rgb(0.6, 0.3, 0.9), // Bright purple
                    _ => Color::from_rgb(0.5, 0.2, 0.75), // Darker purple
                })
                .add_stop(1.0, match status {
                    button::Status::Hovered => Color::from_rgb(0.9, 0.4, 0.7), // Bright pink
                    _ => Color::from_rgb(0.75, 0.3, 0.6), // Darker pink
                })
        ))),
        border: Border {
            color: match status {
                button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.3),
                _ => Color::from_rgba(1.0, 1.0, 1.0, 0.15),
            },
            width: match status {
                button::Status::Hovered => 2.0,
                _ => 1.0,
            },
            radius: 16.0.into(),
        },
        shadow: iced::Shadow {
            color: match status {
                button::Status::Hovered => Color::from_rgba(0.7, 0.3, 0.8, 0.5),
                _ => Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            },
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: match status {
                button::Status::Hovered => 24.0,
                _ => 12.0,
            },
        },
        ..Default::default()
    });


    let button_row = row![
        manage_identities_button,
        settings_button,
    ]
    .spacing(24);

    // Main content column
    let mut content = column![
        welcome,
        Space::with_height(8),
        subtitle,
        Space::with_height(40),
        status_card,
        Space::with_height(60),
        button_row,
    ]
    .align_x(iced::Alignment::Center)
    .spacing(0);

    // Show error message if present
    if let Some(error) = &state.error_message {
        let error_card = container(
            text(error)
                .size(14)
                .style(|_theme| text::Style {
                    color: Some(Color::from_rgb(1.0, 0.4, 0.4)),
                })
        )
        .padding(16)
        .width(Length::Fixed(500.0))
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 0.2, 0.2, 0.1))),
            border: Border {
                color: Color::from_rgba(1.0, 0.4, 0.4, 0.5),
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        });
        
        content = content.push(Space::with_height(40));
        content = content.push(error_card);
    }

    // Center everything
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}
