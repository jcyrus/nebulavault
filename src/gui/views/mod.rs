// Main view modules
pub mod auth;
pub mod main_view;
pub mod host_dialogs;
pub mod identity_dialogs;
pub mod settings;

use iced::Element;
use crate::gui::messages::Message;
use crate::gui::state::{AppState, NebulaVaultState};

pub fn render(state: &NebulaVaultState) -> Element<'_, Message> {
    match &state.state {
        AppState::PasswordEntry => auth::view_password_entry(state),
        AppState::Loading => auth::view_loading(),
        AppState::Ready => main_view::view_main(state),
        AppState::HostDialog => host_dialogs::view_host_dialog(state),
        AppState::DeleteConfirm(host_id) => host_dialogs::view_delete_confirm(state, host_id),
        AppState::IdentityList => identity_dialogs::view_identity_list(state),
        AppState::IdentityDialog => identity_dialogs::view_identity_dialog(state),
        AppState::IdentityDeleteConfirm(identity_id) => identity_dialogs::view_identity_delete_confirm(state, identity_id),
        AppState::Settings => settings::view_settings(state),
        AppState::Error(e) => auth::view_error(e),
    }
}
