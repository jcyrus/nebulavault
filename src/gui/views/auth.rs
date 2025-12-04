use iced::{widget::{button, column, container, text, text_input}, Element, Length};
use crate::gui::messages::Message;
use crate::gui::state::NebulaVaultState;

pub fn view_password_entry(state: &NebulaVaultState) -> Element<'_, Message> {
    let title = text("Unlock Nebula Vault")
        .size(32)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(0.95, 0.95, 0.95)),
        });

    let subtitle = text("Enter your master password to unlock the vault")
        .size(14)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(0.7, 0.7, 0.75)),
        });

    let password_input = text_input("Master password", &state.password_input)
        .on_input(Message::PasswordChanged)
        .on_submit(Message::UnlockVault)
        .secure(true)
        .padding(12)
        .size(16);

    let unlock_button = button(
        text("Unlock")
            .size(16)
            .style(|_theme| text::Style {
                color: Some(iced::Color::WHITE),
            }),
    )
    .on_press(Message::UnlockVault)
    .padding([12, 24]);

    let mut content = column![title, subtitle, password_input, unlock_button]
        .spacing(20)
        .padding(40)
        .max_width(400);

    if let Some(error) = &state.error_message {
        let error_text = text(error)
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

pub fn view_loading() -> Element<'static, Message> {
    let loading_text = text("Loading...")
        .size(24)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(0.7, 0.7, 0.75)),
        });

    container(loading_text)
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

pub fn view_error(error: &str) -> Element<'_, Message> {
    let error_text = text(format!("Error: {}", error))
        .size(18)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(1.0, 0.3, 0.3)),
        });

    container(error_text)
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
