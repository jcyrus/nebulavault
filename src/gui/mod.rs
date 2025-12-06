pub mod messages;
pub mod state;
pub mod app;
pub mod views;
pub mod widgets;

pub use app::NebulaVault;
pub use messages::Message;

pub fn run() -> iced::Result {
    use iced::{window, Theme, Size};
    
    let window_settings = window::Settings {
        size: Size::new(1200.0, 800.0),
        position: window::Position::Centered,
        min_size: Some(Size::new(800.0, 600.0)),
        decorations: true,  // Native window decorations (title bar with controls)
        transparent: false, // Standard opaque window
        level: window::Level::Normal,
        ..Default::default()
    };
    
    iced::application(
        "Nebula Vault",
        NebulaVault::update,
        NebulaVault::view,
    )
    .theme(|_| Theme::Dark)
    .subscription(NebulaVault::subscription)
    .window(window_settings)
    .run_with(|| {
        let app = NebulaVault::new();
        
        // Apply vibrancy effect on macOS
        #[cfg(target_os = "macos")]
        {
            use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
            
            // Note: Window handle needs to be obtained after window creation
            // This will be applied in the first update cycle
        }
        
        app
    })
}
