pub mod messages;
pub mod state;
pub mod app;
pub mod views;

pub use app::NebulaVault;
pub use messages::Message;

pub fn run() -> iced::Result {
    use iced::Theme;
    
    iced::application(
        "Nebula Vault - SSH Connection Manager",
        NebulaVault::update,
        NebulaVault::view,
    )
    .theme(|_| Theme::Dark)
    .subscription(NebulaVault::subscription)
    .run_with(NebulaVault::new)
}
