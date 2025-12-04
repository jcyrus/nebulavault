use crate::models::{Host, Identity};

/// Messages for the application
#[derive(Debug, Clone)]
pub enum Message {
    // Authentication
    PasswordChanged(String),
    UnlockVault,
    VaultUnlockResult(bool, Option<String>),
    
    // Data loading
    HostsLoaded(Vec<Host>),
    HostsLoadResult(bool, Option<String>),
    IdentitiesLoaded(Vec<Identity>),
    
    // Navigation
    Connect(String),
    DecryptAndConnect(crate::models::Host, Vec<u8>),
    ConnectWithCredentials(crate::models::Host, crate::models::IdentityData),
    CancelDialog,
    Disconnect,
    
    // Host management
    ShowAddHostDialog,
    ShowEditHostDialog(String),
    ShowDeleteConfirm(String),
    
    // Host form fields
    HostNameChanged(String),
    HostHostnameChanged(String),
    HostPortChanged(String),
    HostUsernameChanged(String),
    HostIdentityChanged(Option<String>),
    
    // Host actions
    SaveHost,
    HostSaved(bool, Option<String>),
    DeleteHost(String),
    HostDeleted(bool, Option<String>),
    
    // Identity management
    ShowIdentityList,
    ShowAddIdentityDialog,
    ShowEditIdentityDialog(String),
    ShowIdentityDeleteConfirm(String),
    
    // Identity form fields
    IdentityNameChanged(String),
    IdentityTypeChanged(super::state::IdentityType),
    IdentityPasswordChanged(String),
    IdentityKeyChanged(String),
    IdentityPassphraseChanged(String),
    
    // Identity actions
    SaveIdentity,
    IdentitySaved(bool, Option<String>),
    DeleteIdentity(String),
    IdentityDeleted(bool, Option<String>),
    
    // Terminal
    SshConnected(bool, Option<String>),
    TerminalInput(String),
    SendCommand,
    TerminalOutput(String),
}
