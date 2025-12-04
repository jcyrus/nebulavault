use iced::{widget::{button, column, container, row, scrollable, text, Column}, Element, Length};
use crate::gui::messages::Message;
use crate::gui::state::NebulaVaultState;

pub fn view_main(state: &NebulaVaultState) -> Element<'_, Message> {
    let sidebar = render_sidebar(state);
    let main_content = render_main_content(state);

    let content = row![sidebar, main_content].spacing(0);

    container(content)
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
                background: Some(iced::Background::Color(match status {
                    button::Status::Hovered => iced::Color::from_rgb(0.3, 0.6, 0.9),
                    _ => iced::Color::from_rgb(0.2, 0.5, 0.8),
                })),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                text_color: iced::Color::WHITE,
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
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.15, 0.15, 0.18))),
            border: iced::Border {
                color: iced::Color::from_rgb(0.25, 0.25, 0.28),
                width: 0.0,
                radius: 0.0.into(),
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

    // Action buttons
    let edit_button = button(text("✏").size(14))
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

    let delete_button = button(text("🗑").size(14))
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
            background: Some(iced::Background::Color(match status {
                button::Status::Hovered => iced::Color::from_rgb(0.25, 0.25, 0.28),
                _ => iced::Color::from_rgb(0.2, 0.2, 0.23),
            })),
            border: iced::Border {
                color: iced::Color::from_rgb(0.3, 0.3, 0.33),
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .into()
}

fn render_main_content(state: &NebulaVaultState) -> Element<'_, Message> {
    let status_text = text(format!("Loaded {} hosts", state.hosts.len()))
        .size(16)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(0.7, 0.7, 0.75)),
        });

    let placeholder = text("Select a connection to start")
        .size(24)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(0.5, 0.5, 0.55)),
        });

    let manage_identities_button = button(
        text("🔑 Manage Identities")
            .size(16)
            .style(|_theme| text::Style {
                color: Some(iced::Color::WHITE),
            }),
    )
    .on_press(Message::ShowIdentityList)
    .padding([12, 24])
    .style(|_theme, status| button::Style {
        background: Some(iced::Background::Color(match status {
            button::Status::Hovered => iced::Color::from_rgb(0.3, 0.6, 0.9),
            _ => iced::Color::from_rgb(0.2, 0.5, 0.8),
        })),
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        text_color: iced::Color::WHITE,
        ..Default::default()
    });

    let mut content = column![status_text, placeholder, manage_identities_button]
        .spacing(20)
        .padding(40);

    // Show error message if present
    if let Some(error) = &state.error_message {
        let error_text = text(format!("❌ {}", error))
            .size(14)
            .style(|_theme| text::Style {
                color: Some(iced::Color::from_rgb(1.0, 0.3, 0.3)),
            });
        content = content.push(error_text);
    }

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.12))),
            ..Default::default()
        })
        .into()
}
