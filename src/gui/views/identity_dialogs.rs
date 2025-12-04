use iced::{widget::{button, column, container, row, scrollable, text, text_input, Column}, Element, Length};
use crate::gui::messages::Message;
use crate::gui::state::{IdentityType, NebulaVaultState};

pub fn view_identity_list(state: &NebulaVaultState) -> Element<'_, Message> {
    let title_row = row![
        text("Identities")
            .size(24)
            .style(|_theme| text::Style {
                color: Some(iced::Color::from_rgb(0.95, 0.95, 0.95)),
            }),
        button(text("+ Add Identity").size(14))
            .on_press(Message::ShowAddIdentityDialog)
            .padding([8, 16])
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
    .spacing(20)
    .align_y(iced::Alignment::Center);

    let mut identity_list = Column::new().spacing(12);

    if state.identities.is_empty() {
        let empty_text = text("No identities yet. Add one to get started.")
            .size(14)
            .style(|_theme| text::Style {
                color: Some(iced::Color::from_rgb(0.6, 0.6, 0.65)),
            });
        identity_list = identity_list.push(empty_text);
    } else {
        for identity in &state.identities {
            let name_owned = identity.name.clone();
            
            let name_text = text(name_owned)
                .size(16)
                .style(|_theme| text::Style {
                    color: Some(iced::Color::from_rgb(0.95, 0.95, 0.95)),
                });

            let type_icon = text("🔑")
                .size(16);

            let item_row = row![type_icon, name_text]
                .spacing(12)
                .align_y(iced::Alignment::Center);

            let item_button = button(item_row)
                .padding(12)
                .width(Length::Fill)
                .style(|_theme, status| button::Style {
                    background: Some(iced::Background::Color(match status {
                        button::Status::Hovered => iced::Color::from_rgb(0.2, 0.2, 0.23),
                        _ => iced::Color::from_rgb(0.15, 0.15, 0.18),
                    })),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.3, 0.3, 0.33),
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                });

            identity_list = identity_list.push(item_button);
        }
    }

    let back_button = button(text("← Back").size(14))
        .on_press(Message::CancelDialog)
        .padding([10, 20]);

    let content = column![
        title_row,
        scrollable(identity_list).height(Length::Fill),
        back_button
    ]
    .spacing(20)
    .padding(40)
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.12))),
            ..Default::default()
        })
        .into()
}

pub fn view_identity_dialog(state: &NebulaVaultState) -> Element<'_, Message> {
    let title = text(if state.identity_form.editing_id.is_some() {
        "Edit Identity"
    } else {
        "Add New Identity"
    })
    .size(24)
    .style(|_theme| text::Style {
        color: Some(iced::Color::from_rgb(0.95, 0.95, 0.95)),
    });

    let name_input = column![
        text("Name").size(14),
        text_input("My SSH Key", &state.identity_form.name)
            .on_input(Message::IdentityNameChanged)
            .padding(10)
    ]
    .spacing(8);

    // Type selector
    let is_password = state.identity_form.identity_type == IdentityType::Password;
    let is_ssh_key = state.identity_form.identity_type == IdentityType::SshKey;
    
    let type_selector = column![
        text("Type").size(14),
        row![
            button(text("Password").size(14))
                .padding([8, 16])
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(
                        if is_password {
                            iced::Color::from_rgb(0.2, 0.5, 0.8)
                        } else {
                            iced::Color::from_rgb(0.2, 0.2, 0.23)
                        }
                    )),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    text_color: iced::Color::WHITE,
                    ..Default::default()
                })
                .on_press(Message::IdentityTypeChanged(IdentityType::Password)),
            button(text("SSH Key").size(14))
                .padding([8, 16])
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(
                        if is_ssh_key {
                            iced::Color::from_rgb(0.2, 0.5, 0.8)
                        } else {
                            iced::Color::from_rgb(0.2, 0.2, 0.23)
                        }
                    )),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    text_color: iced::Color::WHITE,
                    ..Default::default()
                })
                .on_press(Message::IdentityTypeChanged(IdentityType::SshKey)),
        ]
        .spacing(12)
    ]
    .spacing(8);

    // Conditional fields based on type
    let mut form_fields = column![name_input, type_selector].spacing(20);

    match state.identity_form.identity_type {
        IdentityType::Password => {
            let password_input = column![
                text("Password").size(14),
                text_input("Enter password", &state.identity_form.password)
                    .on_input(Message::IdentityPasswordChanged)
                    .secure(true)
                    .padding(10)
            ]
            .spacing(8);
            form_fields = form_fields.push(password_input);
        }
        IdentityType::SshKey => {
            let key_instruction = text("Note: Paste your private key as a single line (replace newlines with \\n) or use the full file path")
                .size(12)
                .style(|_theme| text::Style {
                    color: Some(iced::Color::from_rgb(0.7, 0.7, 0.75)),
                });

            let key_input = column![
                text("Private Key or File Path").size(14),
                key_instruction,
                text_input("Paste key or enter ~/.ssh/id_rsa", &state.identity_form.key)
                    .on_input(Message::IdentityKeyChanged)
                    .padding(10)
            ]
            .spacing(8);

            let passphrase_input = column![
                text("Passphrase (optional)").size(14),
                text_input("Key passphrase", &state.identity_form.passphrase)
                    .on_input(Message::IdentityPassphraseChanged)
                    .secure(true)
                    .padding(10)
            ]
            .spacing(8);

            form_fields = form_fields.push(key_input).push(passphrase_input);
        }
    }

    let buttons = row![
        button(text("Cancel").size(14))
            .on_press(Message::CancelDialog)
            .padding([10, 20]),
        button(text("Save").size(14))
            .on_press(Message::SaveIdentity)
            .padding([10, 20])
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
    .spacing(12);

    let dialog_content = column![
        title,
        form_fields,
        buttons
    ]
    .spacing(20)
    .padding(30)
    .max_width(500);

    container(dialog_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.8))),
            ..Default::default()
        })
        .into()
}

pub fn view_identity_delete_confirm(_state: &NebulaVaultState, _identity_id: &str) -> Element<'static, Message> {
    let title = text("Delete Identity?")
        .size(24)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(1.0, 0.3, 0.3)),
        });

    let buttons = row![
        button(text("Cancel").size(14))
            .on_press(Message::CancelDialog)
            .padding([10, 20]),
        button(text("Delete").size(14))
            .padding([10, 20]),
    ]
    .spacing(12);

    let content = column![title, buttons]
        .spacing(20)
        .padding(40);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.8))),
            ..Default::default()
        })
        .into()
}
