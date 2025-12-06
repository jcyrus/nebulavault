// Example launcher view composition with glassmorphism

use iced::{
    widget::{column, container, scrollable, text, Column, Space},
    Element, Length,
};
use crate::gui::{messages::Message, state::NebulaVaultState};
use super::widgets::glass::{self, GlassContainer};

pub fn view_launcher(state: &NebulaVaultState) -> Element<'_, Message> {
    // Search input with neon glow
    let search_input = glass::neon_input::styled(
        "Search servers...",
        &state.search_query,
        Message::SearchQueryChanged,
    )
    .width(Length::Fill);

    let search_container = container(search_input)
        .width(Length::Fill)
        .padding([20, 40]);

    // Server list
    let mut server_list = Column::new().spacing(12).padding([0, 40]);

    for host in &state.hosts {
        let is_selected = state.selected_host_id.as_ref() == Some(&host.id);
        let is_hovered = state.hovered_host_id.as_ref() == Some(&host.id);

        let card = glass::server_card(
            &host.name,
            &format!("{}:{}", host.hostname, host.port),
            &host.tags,
            is_selected,
            is_hovered,
        );

        server_list = server_list.push(card);
    }

    let scrollable_list = scrollable(server_list)
        .height(Length::Fill);

    // Status bar
    let status_text = text(format!(
        "{} servers | {} selected",
        state.hosts.len(),
        if state.selected_host_id.is_some() { "1" } else { "0" }
    ))
    .size(12)
    .style(|_theme| text::Style {
        color: Some(iced::Color::from_rgba(0.7, 0.7, 0.7, 0.8)),
    });

    let status_bar = GlassContainer::new(status_text)
        .width(Length::Fill);

    // Main layout
    let content = column![
        search_container,
        scrollable_list,
        Space::with_height(20),
        status_bar,
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // Wrap in container with deep space background
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(
                iced::Color::from_rgb(0.059, 0.090, 0.165) // Deep Space Navy
            )),
            ..Default::default()
        })
        .into()
}

/* 
NOTES FOR SHADER INTEGRATION:

To layer the shader background, you would:

1. Create a custom widget that renders the shader as a background layer
2. Use iced::widget::Stack to layer components:

```rust
use iced::widget::Stack;

Stack::new()
    .push(nebula_shader_background()) // Bottom layer: shader
    .push(view_launcher(state))        // Top layer: UI
```

3. The shader widget would use iced's canvas or a custom primitive
   to render the WGSL shader to a full-screen quad

4. For window vibrancy, configure in main.rs:

```rust
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

fn main() {
    let settings = iced::Settings {
        window: iced::window::Settings {
            decorations: false, // Frameless
            transparent: true,  // For vibrancy
            ..Default::default()
        },
        ..Default::default()
    };

    // After window creation:
    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Failed to apply vibrancy");
}
```

*/
