use crate::models::{Host, Identity};
use crate::vault::Vault;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Application state
#[derive(Debug, Clone)]
pub enum AppState {
    PasswordEntry,
    Loading,
    Ready,
    HostDialog,
    DeleteConfirm(String),
    IdentityList,
    IdentityDialog,
    IdentityDeleteConfirm(String),
    Settings,
    Error(String),
}

/// Identity type selector
#[derive(Debug, Clone, PartialEq)]
pub enum IdentityType {
    Password,
    SshKey,
}

/// Host form data
#[derive(Debug, Clone, Default)]
pub struct HostForm {
    pub editing_id: Option<String>,
    pub name: String,
    pub hostname: String,
    pub port: String,
    pub username: String,
    pub identity_id: Option<String>,
}

impl HostForm {
    pub fn new() -> Self {
        Self {
            port: "22".to_string(),
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.editing_id = None;
        self.name.clear();
        self.hostname.clear();
        self.port = "22".to_string();
        self.username.clear();
        self.identity_id = None;
    }
}

/// Identity form data
#[derive(Debug, Clone, Default)]
pub struct IdentityForm {
    pub editing_id: Option<String>,
    pub name: String,
    pub identity_type: IdentityType,
    pub password: String,
    pub key: String,
    pub passphrase: String,
}

impl Default for IdentityType {
    fn default() -> Self {
        IdentityType::Password
    }
}

impl IdentityForm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.editing_id = None;
        self.name.clear();
        self.identity_type = IdentityType::Password;
        self.password.clear();
        self.key.clear();
        self.passphrase.clear();
    }
}

/// Main application state
pub struct NebulaVaultState {
    pub state: AppState,
    pub password_input: String,
    pub vault: Option<Vault>,
    pub db_pool: Option<SqlitePool>,
    pub hosts: Vec<Host>,
    pub identities: Vec<Identity>,
    pub error_message: Option<String>,
    
    // Forms
    pub host_form: HostForm,
    pub identity_form: IdentityForm,
    
    // Terminal preference
    pub terminal_preference: crate::terminal_launcher::TerminalApp,
    
    // SSH Session (for cleanup)
    pub ssh_session: Option<Arc<Mutex<crate::ssh::SshSession>>>,
}

impl NebulaVaultState {
    pub fn new() -> Self {
        Self {
            state: AppState::PasswordEntry,
            password_input: String::new(),
            vault: None,
            db_pool: None,
            hosts: Vec::new(),
            identities: Vec::new(),
            error_message: None,
            host_form: HostForm::new(),
            identity_form: IdentityForm::new(),
            terminal_preference: crate::terminal_launcher::TerminalApp::default(),
            ssh_session: None,
        }
    }
}
