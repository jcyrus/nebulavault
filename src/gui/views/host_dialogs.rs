use iced::{widget::{button, column, container, row, scrollable, text, text_input}, Element, Length};
use crate::gui::messages::Message;
use crate::gui::state::NebulaVaultState;

pub fn view_host_dialog(state: &NebulaVaultState) -> Element<'_, Message> {
    let title = text(if state.host_form.editing_id.is_some() {
        "Edit Host"
    } else {
        "Add New Host"
    })
    .size(24)
    .style(|_theme| text::Style {
        color: Some(iced::Color::from_rgb(0.95, 0.95, 0.95)),
    });

    let name_input = column![
        text("Name").size(14),
        text_input("My Server", &state.host_form.name)
            .on_input(Message::HostNameChanged)
            .padding(10)
    ]
    .spacing(8);

    let hostname_input = column![
        text("Hostname").size(14),
        text_input("example.com", &state.host_form.hostname)
            .on_input(Message::HostHostnameChanged)
            .padding(10)
    ]
    .spacing(8);

    let port_input = column![
        text("Port").size(14),
        text_input("22", &state.host_form.port)
            .on_input(Message::HostPortChanged)
            .padding(10)
    ]
    .spacing(8);

    let username_input = column![
        text("Username").size(14),
        text_input("root", &state.host_form.username)
            .on_input(Message::HostUsernameChanged)
            .padding(10)
    ]
    .spacing(8);

    // Identity selector
    let identity_selector = if state.identities.is_empty() {
        column![
            text("Identity (optional)").size(14),
            text("No identities available. Create one first.")
                .size(12)
                .style(|_theme| text::Style {
                    color: Some(iced::Color::from_rgb(0.6, 0.6, 0.65)),
                })
        ]
        .spacing(8)
    } else {
        let mut identity_buttons = row![].spacing(8);
        
        // "None" button
        let is_none_selected = state.host_form.identity_id.is_none();
        identity_buttons = identity_buttons.push(
            button(text("None").size(12))
                .padding([6, 12])
                .on_press(Message::HostIdentityChanged(None))
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(
                        if is_none_selected {
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
        );

        // Identity buttons
        for identity in &state.identities {
            let id = identity.id.clone();
            let name = identity.name.clone();
            let is_selected = state.host_form.identity_id.as_ref() == Some(&identity.id);
            
            identity_buttons = identity_buttons.push(
                button(text(name).size(12))
                    .padding([6, 12])
                    .on_press(Message::HostIdentityChanged(Some(id)))
                    .style(move |_theme, _status| button::Style {
                        background: Some(iced::Background::Color(
                            if is_selected {
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
            );
        }

        column![
            text("Identity (optional)").size(14),
            scrollable(identity_buttons).width(Length::Fill)
        ]
        .spacing(8)
    };

    let buttons = row![
        button(text("Cancel").size(14))
            .on_press(Message::CancelDialog)
            .padding([10, 20]),
        button(text("Save").size(14))
            .on_press(Message::SaveHost)
            .padding([10, 20]),
    ]
    .spacing(12);

    let dialog_content = column![
        title,
        name_input,
        hostname_input,
        port_input,
        username_input,
        identity_selector,
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

pub fn view_delete_confirm<'a>(state: &'a NebulaVaultState, host_id: &'a str) -> Element<'a, Message> {
    let host_name = state
        .hosts
        .iter()
        .find(|h| h.id == host_id)
        .map(|h| h.name.as_str())
        .unwrap_or("this host");

    let title = text("Delete Host?")
        .size(24)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(1.0, 0.3, 0.3)),
        });

    let message = text(format!("Are you sure you want to delete \"{}\"?", host_name))
        .size(16)
        .style(|_theme| text::Style {
            color: Some(iced::Color::from_rgb(0.9, 0.9, 0.9)),
        });

    let host_id_owned = host_id.to_string();
    let buttons = row![
        button(text("Cancel").size(14))
            .on_press(Message::CancelDialog)
            .padding([10, 20]),
        button(text("Delete").size(14))
            .on_press(Message::DeleteHost(host_id_owned))
            .padding([10, 20]),
    ]
    .spacing(12);

    let dialog_content = column![title, message, buttons]
        .spacing(20)
        .padding(30)
        .max_width(400);

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
