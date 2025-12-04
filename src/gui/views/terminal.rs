use iced::{widget::{button, column, container, row, scrollable, text, text_input}, Element, Length};
use crate::gui::messages::Message;
use crate::gui::state::NebulaVaultState;

pub fn view_terminal(state: &NebulaVaultState) -> Element<'_, Message> {
    let host_name = state
        .terminal.active_host_id
        .as_ref()
        .and_then(|id| state.hosts.iter().find(|h| &h.id == id))
        .map(|h| h.name.as_str())
        .unwrap_or("Unknown");

    let header = row![
        text(format!("Connected to: {}", host_name))
            .size(18)
            .style(|_theme| text::Style {
                color: Some(iced::Color::from_rgb(0.9, 0.9, 0.9)),
            }),
        button(text("Disconnect").size(14))
            .on_press(Message::Disconnect)
            .padding([8, 16]),
    ]
    .spacing(20)
    .align_y(iced::Alignment::Center)
    .padding(16);

    let output_area = scrollable(
        text(&state.terminal.output)
            .font(iced::Font::MONOSPACE)
            .size(14)
            .style(|_theme| text::Style {
                color: Some(iced::Color::from_rgb(0.9, 0.9, 0.9)),
            }),
    )
    .height(Length::Fill);

    let input_field = text_input("Enter command...", &state.terminal.input)
        .on_input(Message::TerminalInput)
        .on_submit(Message::SendCommand)
        .font(iced::Font::MONOSPACE)
        .padding(12)
        .size(14);

    let terminal_content = column![header, output_area, input_field]
        .spacing(0)
        .height(Length::Fill);

    container(terminal_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.05, 0.05, 0.08))),
            ..Default::default()
        })
        .into()
}
