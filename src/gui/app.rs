use iced::{Element, Subscription, Task};
use super::messages::Message;
use super::state::{AppState, NebulaVaultState};
use crate::{db, models, ssh};

const DB_PATH: &str = "nebulavault.db";

pub struct NebulaVault {
    state: NebulaVaultState,
}

impl NebulaVault {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: NebulaVaultState::new(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PasswordChanged(password) => {
                self.state.password_input = password;
                self.state.error_message = None;
                Task::none()
            }

            Message::UnlockVault => {
                if self.state.password_input.is_empty() {
                    self.state.error_message = Some("Password cannot be empty".to_string());
                    return Task::none();
                }

                self.state.state = AppState::Loading;
                let password = self.state.password_input.clone();

                Task::perform(
                    async move {
                        let mut vault = crate::vault::Vault::new();
                        if let Err(e) = vault.derive_key(&password) {
                            return (false, Some(format!("Failed to derive key: {}", e)));
                        }

                        match db::init_db(DB_PATH).await {
                            Ok(_pool) => (true, None),
                            Err(e) => (false, Some(format!("Failed to initialize database: {}", e))),
                        }
                    },
                    |(success, error)| Message::VaultUnlockResult(success, error),
                )
            }

            Message::VaultUnlockResult(success, error) => {
                if success {
                    let password = self.state.password_input.clone();
                    self.state.password_input.clear();
                    
                    let mut vault = crate::vault::Vault::new();
                    if let Err(e) = vault.derive_key(&password) {
                        self.state.state = AppState::PasswordEntry;
                        self.state.error_message = Some(format!("Failed to unlock: {}", e));
                        return Task::none();
                    }
                    
                    self.state.vault = Some(vault);
                    self.state.state = AppState::Ready;
                    
                    Task::perform(
                        async move {
                            match db::init_db(DB_PATH).await {
                                Ok(pool) => {
                                    match db::get_all_hosts(&pool).await {
                                        Ok(_hosts) => (true, None),
                                        Err(e) => (false, Some(format!("Failed to load hosts: {}", e))),
                                    }
                                }
                                Err(e) => (false, Some(format!("Database error: {}", e))),
                            }
                        },
                        |(success, error)| Message::HostsLoadResult(success, error),
                    )
                } else {
                    self.state.state = AppState::PasswordEntry;
                    self.state.error_message = error;
                    Task::none()
                }
            }

            Message::HostsLoadResult(success, _error) => {
                if success {
                    Task::perform(
                        async move {
                            match db::init_db(DB_PATH).await {
                                Ok(pool) => {
                                    match db::get_all_hosts(&pool).await {
                                        Ok(hosts) => hosts,
                                        Err(_) => Vec::new(),
                                    }
                                }
                                Err(_) => Vec::new(),
                            }
                        },
                        Message::HostsLoaded,
                    )
                } else {
                    Task::none()
                }
            }

            Message::HostsLoaded(hosts) => {
                self.state.hosts = hosts;
                Task::perform(
                    async move {
                        match db::init_db(DB_PATH).await {
                            Ok(pool) => {
                                match db::get_all_identities(&pool).await {
                                    Ok(identities) => identities,
                                    Err(_) => Vec::new(),
                                }
                            }
                            Err(_) => Vec::new(),
                        }
                    },
                    Message::IdentitiesLoaded,
                )
            }

            Message::IdentitiesLoaded(identities) => {
                self.state.identities = identities;
                Task::none()
            }

            // Host management
            Message::ShowAddHostDialog => {
                self.state.host_form.clear();
                self.state.state = AppState::HostDialog;
                Task::none()
            }

            Message::ShowEditHostDialog(host_id) => {
                if let Some(host) = self.state.hosts.iter().find(|h| h.id == host_id) {
                    self.state.host_form.editing_id = Some(host_id);
                    self.state.host_form.name = host.name.clone();
                    self.state.host_form.hostname = host.hostname.clone();
                    self.state.host_form.port = host.port.to_string();
                    self.state.host_form.username = host.username.clone();
                    self.state.host_form.identity_id = host.identity_id.clone();
                    self.state.state = AppState::HostDialog;
                }
                Task::none()
            }

            Message::ShowDeleteConfirm(host_id) => {
                self.state.state = AppState::DeleteConfirm(host_id);
                Task::none()
            }

            Message::CancelDialog => {
                self.state.state = AppState::Ready;
                Task::none()
            }

            Message::HostNameChanged(name) => {
                self.state.host_form.name = name;
                Task::none()
            }

            Message::HostHostnameChanged(hostname) => {
                self.state.host_form.hostname = hostname;
                Task::none()
            }

            Message::HostPortChanged(port) => {
                self.state.host_form.port = port;
                Task::none()
            }

            Message::HostUsernameChanged(username) => {
                self.state.host_form.username = username;
                Task::none()
            }

            Message::HostIdentityChanged(identity_id) => {
                self.state.host_form.identity_id = identity_id;
                Task::none()
            }

            Message::SaveHost => {
                let name = self.state.host_form.name.clone();
                let hostname = self.state.host_form.hostname.clone();
                let port = self.state.host_form.port.parse::<i64>().unwrap_or(22);
                let username = self.state.host_form.username.clone();
                let identity_id = self.state.host_form.identity_id.clone();

                self.state.state = AppState::Loading;

                Task::perform(
                    async move {
                        let pool = match db::init_db(DB_PATH).await {
                            Ok(p) => p,
                            Err(e) => return (false, Some(format!("Database error: {}", e))),
                        };

                        match db::create_host(&pool, None, identity_id, name, hostname, port, username, None).await {
                            Ok(_) => (true, None),
                            Err(e) => (false, Some(format!("Failed to create host: {}", e))),
                        }
                    },
                    |(success, error)| Message::HostSaved(success, error),
                )
            }

            Message::HostSaved(success, error) => {
                if success {
                    self.state.state = AppState::Ready;
                    Task::perform(
                        async move {
                            match db::init_db(DB_PATH).await {
                                Ok(pool) => {
                                    match db::get_all_hosts(&pool).await {
                                        Ok(hosts) => hosts,
                                        Err(_) => Vec::new(),
                                    }
                                }
                                Err(_) => Vec::new(),
                            }
                        },
                        Message::HostsLoaded,
                    )
                } else {
                    self.state.state = AppState::Ready;
                    self.state.error_message = error;
                    Task::none()
                }
            }

            Message::DeleteHost(host_id) => {
                self.state.state = AppState::Loading;

                Task::perform(
                    async move {
                        let pool = match db::init_db(DB_PATH).await {
                            Ok(p) => p,
                            Err(e) => return (false, Some(format!("Database error: {}", e))),
                        };

                        match db::delete_host(&pool, &host_id).await {
                            Ok(_) => (true, None),
                            Err(e) => (false, Some(format!("Failed to delete host: {}", e))),
                        }
                    },
                    |(success, error)| Message::HostDeleted(success, error),
                )
            }

            Message::HostDeleted(success, error) => {
                if success {
                    self.state.state = AppState::Ready;
                    Task::perform(
                        async move {
                            match db::init_db(DB_PATH).await {
                                Ok(pool) => {
                                    match db::get_all_hosts(&pool).await {
                                        Ok(hosts) => hosts,
                                        Err(_) => Vec::new(),
                                    }
                                }
                                Err(_) => Vec::new(),
                            }
                        },
                        Message::HostsLoaded,
                    )
                } else {
                    self.state.state = AppState::Ready;
                    self.state.error_message = error;
                    Task::none()
                }
            }

            // Identity management - simplified for now
            Message::ShowIdentityList => {
                self.state.state = AppState::IdentityList;
                Task::none()
            }

            Message::ShowAddIdentityDialog => {
                self.state.identity_form.clear();
                self.state.state = AppState::IdentityDialog;
                Task::none()
            }

            Message::ShowEditIdentityDialog(_identity_id) => {
                // TODO: Implement edit
                Task::none()
            }

            Message::ShowIdentityDeleteConfirm(identity_id) => {
                self.state.state = AppState::IdentityDeleteConfirm(identity_id);
                Task::none()
            }

            Message::IdentityNameChanged(name) => {
                self.state.identity_form.name = name;
                Task::none()
            }

            Message::IdentityTypeChanged(identity_type) => {
                self.state.identity_form.identity_type = identity_type;
                Task::none()
            }

            Message::IdentityPasswordChanged(password) => {
                self.state.identity_form.password = password;
                Task::none()
            }

            Message::IdentityKeyChanged(key) => {
                self.state.identity_form.key = key;
                Task::none()
            }

            Message::IdentityPassphraseChanged(passphrase) => {
                self.state.identity_form.passphrase = passphrase;
                Task::none()
            }

            Message::SaveIdentity => {
                // Encrypt identity data synchronously (vault is in main thread)
                let vault = match &self.state.vault {
                    Some(v) => v,
                    None => {
                        self.state.error_message = Some("Vault not available".to_string());
                        self.state.state = AppState::IdentityList;
                        return Task::none();
                    }
                };

                let name = self.state.identity_form.name.clone();
                
                // Create identity data based on type
                let identity_data = match self.state.identity_form.identity_type {
                    super::state::IdentityType::Password => {
                        models::IdentityData::Password {
                            password: self.state.identity_form.password.clone(),
                        }
                    }
                    super::state::IdentityType::SshKey => {
                        // Check if it's a file path or raw key
                        let key_input = self.state.identity_form.key.trim();
                        let private_key = if key_input.starts_with("~/") || key_input.starts_with('/') {
                            // It's a file path - read the file
                            let expanded_path = if key_input.starts_with("~/") {
                                if let Some(home) = std::env::var("HOME").ok() {
                                    key_input.replacen("~/", &format!("{}/", home), 1)
                                } else {
                                    self.state.error_message = Some("Could not expand ~ in path".to_string());
                                    self.state.state = AppState::IdentityDialog;
                                    return Task::none();
                                }
                            } else {
                                key_input.to_string()
                            };

                            match std::fs::read_to_string(&expanded_path) {
                                Ok(content) => content,
                                Err(e) => {
                                    self.state.error_message = Some(format!("Failed to read key file: {}", e));
                                    self.state.state = AppState::IdentityDialog;
                                    return Task::none();
                                }
                            }
                        } else {
                            // It's the raw key content
                            key_input.to_string()
                        };

                        models::IdentityData::SshKey {
                            private_key,
                            passphrase: if self.state.identity_form.passphrase.is_empty() {
                                None
                            } else {
                                Some(self.state.identity_form.passphrase.clone())
                            },
                        }
                    }
                };

                // Encrypt the identity data
                let encrypted_data = match vault.encrypt_identity(&identity_data) {
                    Ok(data) => data,
                    Err(e) => {
                        self.state.error_message = Some(format!("Encryption failed: {}", e));
                        self.state.state = AppState::IdentityList;
                        return Task::none();
                    }
                };

                self.state.state = AppState::Loading;

                // Now save to database asynchronously
                Task::perform(
                    async move {
                        let pool = match db::init_db(DB_PATH).await {
                            Ok(p) => p,
                            Err(e) => return (false, Some(format!("Database error: {}", e))),
                        };

                        match db::create_identity(&pool, name, encrypted_data).await {
                            Ok(_) => (true, None),
                            Err(e) => (false, Some(format!("Failed to save identity: {}", e))),
                        }
                    },
                    |(success, error)| Message::IdentitySaved(success, error),
                )
            }

            Message::IdentitySaved(success, error) => {
                if success {
                    self.state.state = AppState::IdentityList;
                    // Reload identities
                    Task::perform(
                        async move {
                            match db::init_db(DB_PATH).await {
                                Ok(pool) => {
                                    match db::get_all_identities(&pool).await {
                                        Ok(identities) => identities,
                                        Err(_) => Vec::new(),
                                    }
                                }
                                Err(_) => Vec::new(),
                            }
                        },
                        Message::IdentitiesLoaded,
                    )
                } else {
                    self.state.state = AppState::IdentityList;
                    self.state.error_message = error;
                    Task::none()
                }
            }

            Message::DeleteIdentity(identity_id) => {
                self.state.state = AppState::Loading;

                Task::perform(
                    async move {
                        let pool = match db::init_db(DB_PATH).await {
                            Ok(p) => p,
                            Err(e) => return (false, Some(format!("Database error: {}", e))),
                        };

                        match db::delete_identity(&pool, &identity_id).await {
                            Ok(_) => (true, None),
                            Err(e) => (false, Some(format!("Failed to delete identity: {}", e))),
                        }
                    },
                    |(success, error)| Message::IdentityDeleted(success, error),
                )
            }

            Message::IdentityDeleted(success, error) => {
                if success {
                    self.state.state = AppState::IdentityList;
                    // Reload identities
                    Task::perform(
                        async move {
                            match db::init_db(DB_PATH).await {
                                Ok(pool) => {
                                    match db::get_all_identities(&pool).await {
                                        Ok(identities) => identities,
                                        Err(_) => Vec::new(),
                                    }
                                }
                                Err(_) => Vec::new(),
                            }
                        },
                        Message::IdentitiesLoaded,
                    )
                } else {
                    self.state.state = AppState::IdentityList;
                    self.state.error_message = error;
                    Task::none()
                }
            }

            // Terminal/SSH
            Message::Connect(host_id) => {
                if let Some(host) = self.state.hosts.iter().find(|h| h.id == host_id).cloned() {
                    self.state.terminal.active_host_id = Some(host_id.clone());
                    self.state.terminal.output.clear();
                    self.state.terminal.input.clear();
                    
                    self.state.state = AppState::Loading;

                    // Load encrypted identity from database
                    if let Some(identity_id) = host.identity_id.clone() {
                        Task::perform(
                            async move {
                                let pool = match db::init_db(DB_PATH).await {
                                    Ok(p) => p,
                                    Err(e) => return (None, None, Some(format!("Database error: {}", e))),
                                };

                                match db::get_identity(&pool, &identity_id).await {
                                    Ok(Some(identity)) => (Some(host), Some(identity.encrypted_data), None),
                                    Ok(None) => (None, None, Some("Identity not found".to_string())),
                                    Err(e) => (None, None, Some(format!("Failed to load identity: {}", e))),
                                }
                            },
                            |(host_opt, encrypted_data_opt, error_opt)| {
                                if let (Some(host), Some(encrypted_data)) = (host_opt, encrypted_data_opt) {
                                    Message::DecryptAndConnect(host, encrypted_data)
                                } else {
                                    Message::SshConnected(false, error_opt)
                                }
                            },
                        )
                    } else {
                        // No identity configured
                        self.state.state = AppState::Ready;
                        self.state.error_message = Some("No identity configured for this host".to_string());
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            }

            Message::DecryptAndConnect(host, encrypted_data) => {
                // Decrypt identity synchronously (vault is not Send)
                let vault = match &self.state.vault {
                    Some(v) => v,
                    None => {
                        self.state.state = AppState::Error("Vault not available".to_string());
                        return Task::none();
                    }
                };

                match vault.decrypt_identity(&encrypted_data) {
                    Ok(identity_data) => {
                        // Now connect with decrypted credentials
                        Task::done(Message::ConnectWithCredentials(host, identity_data))
                    }
                    Err(e) => {
                        self.state.state = AppState::Ready;
                        self.state.error_message = Some(format!("Failed to decrypt identity: {}", e));
                        Task::none()
                    }
                }
            }

            Message::ConnectWithCredentials(host, identity_data) => {
                // Connect to SSH with decrypted credentials
                Task::perform(
                    async move {
                        let result = match identity_data {
                            models::IdentityData::Password { password } => {
                                ssh::SshSession::connect_password(
                                    &host.hostname,
                                    host.port as u16,
                                    &host.username,
                                    &password,
                                ).await
                            }
                            models::IdentityData::SshKey { private_key, passphrase } => {
                                ssh::SshSession::connect_key(
                                    &host.hostname,
                                    host.port as u16,
                                    &host.username,
                                    &private_key,
                                    passphrase.as_deref(),
                                ).await
                            }
                        };

                        match result {
                            Ok(_session) => {
                                // TODO: Store session for data streaming
                                (true, None)
                            }
                            Err(e) => (false, Some(format!("SSH connection failed: {}", e))),
                        }
                    },
                    |(success, error)| Message::SshConnected(success, error),
                )
            }

            Message::SshConnected(success, error) => {
                if success {
                    self.state.state = AppState::Terminal;
                    self.state.terminal.output.push_str("Connected successfully!\n\n");
                    // TODO: Start SSH data streaming subscription
                    Task::none()
                } else {
                    self.state.state = AppState::Ready;
                    if let Some(err) = error {
                        self.state.error_message = Some(err);
                    }
                    Task::none()
                }
            }

            Message::TerminalInput(input) => {
                self.state.terminal.input = input;
                Task::none()
            }

            Message::SendCommand => {
                let command = self.state.terminal.input.trim().to_string();
                if !command.is_empty() {
                    self.state.terminal.output.push_str(&format!("$ {}\n", command));
                    
                    // TODO: Send command to SSH session
                    // For now, show a message that streaming isn't implemented yet
                    self.state.terminal.output.push_str(
                        "⚠️  SSH data streaming not yet implemented.\n\
                         Connection established, but terminal I/O requires subscription-based streaming.\n\n"
                    );
                    
                    self.state.terminal.input.clear();
                }
                Task::none()
            }

            Message::TerminalOutput(output) => {
                self.state.terminal.output.push_str(&output);
                Task::none()
            }

            Message::Disconnect => {
                self.state.state = AppState::Ready;
                self.state.terminal.clear();
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        super::views::render(&self.state)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
