use anyhow::{anyhow, Result};
use russh::client::{self, Handle};
use russh::*;
use russh_keys::key::PublicKey;
use std::sync::Arc;

/// SSH client handler
struct Client;

#[async_trait::async_trait]
impl client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &PublicKey,
    ) -> Result<bool, Self::Error> {
        // For now, accept all server keys
        // TODO: Implement proper host key verification
        Ok(true)
    }
}

/// SSH session wrapper
pub struct SshSession {
    pub host_id: String,
    pub hostname: String,
    pub username: String,
    handle: Handle<Client>,
    channel: Option<Channel<client::Msg>>,
}

impl SshSession {
    /// Connect to SSH server with password authentication
    pub async fn connect_password(
        hostname: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<Self> {
        let config = client::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(300)),
            ..<_>::default()
        };

        let config = Arc::new(config);
        let sh = Client;

        // Connect to the server
        let mut session = client::connect(config, (hostname, port), sh)
            .await
            .map_err(|e| anyhow!("Failed to connect: {}", e))?;

        // Authenticate with password
        let auth_res = session
            .authenticate_password(username.to_string(), password.to_string())
            .await
            .map_err(|e| anyhow!("Authentication failed: {}", e))?;

        if !auth_res {
            return Err(anyhow!("Authentication failed: invalid credentials"));
        }

        Ok(Self {
            host_id: String::new(),
            hostname: hostname.to_string(),
            username: username.to_string(),
            handle: session,
            channel: None,
        })
    }

    /// Connect to SSH server with public key authentication
    pub async fn connect_key(
        hostname: &str,
        port: u16,
        username: &str,
        private_key: &str,
        passphrase: Option<&str>,
    ) -> Result<Self> {
        let config = client::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(300)),
            ..<_>::default()
        };

        let config = Arc::new(config);
        let sh = Client;

        // Connect to the server
        let mut session = client::connect(config, (hostname, port), sh)
            .await
            .map_err(|e| anyhow!("Failed to connect: {}", e))?;

        // Parse the private key (with optional passphrase)
        let key_pair = russh_keys::decode_secret_key(private_key, passphrase)
            .map_err(|e| anyhow!("Failed to parse private key: {}", e))?;

        // Authenticate with key
        let auth_res = session
            .authenticate_publickey(username.to_string(), Arc::new(key_pair))
            .await
            .map_err(|e| anyhow!("Authentication failed: {}", e))?;

        if !auth_res {
            return Err(anyhow!("Authentication failed: key rejected"));
        }

        Ok(Self {
            host_id: String::new(),
            hostname: hostname.to_string(),
            username: username.to_string(),
            handle: session,
            channel: None,
        })
    }

    /// Open a shell channel
    pub async fn open_shell(&mut self) -> Result<()> {
        let channel = self
            .handle
            .channel_open_session()
            .await
            .map_err(|e| anyhow!("Failed to open channel: {}", e))?;

        // Request a PTY
        channel
            .request_pty(
                false, // want_reply
                "xterm",
                80,  // cols
                24,  // rows
                0,   // width (pixels)
                0,   // height (pixels)
                &[], // terminal modes
            )
            .await
            .map_err(|e| anyhow!("Failed to request PTY: {}", e))?;

        // Request shell
        channel
            .request_shell(false)
            .await
            .map_err(|e| anyhow!("Failed to request shell: {}", e))?;

        self.channel = Some(channel);
        Ok(())
    }

    /// Send data to the channel
    pub async fn send_data(&mut self, data: &str) -> Result<()> {
        if let Some(channel) = &mut self.channel {
            channel
                .data(data.as_bytes())
                .await
                .map_err(|e| anyhow!("Failed to send data: {}", e))?;
            Ok(())
        } else {
            Err(anyhow!("No active channel"))
        }
    }

    /// Read available data from the channel
    pub async fn read_data(&mut self) -> Result<Option<String>> {
        // This is a simplified version
        // In a real implementation, we'd use a subscription to continuously read
        // For now, we'll return None and handle this in the subscription
        Ok(None)
    }

    /// Close the session
    pub async fn close(mut self) -> Result<()> {
        if let Some(channel) = self.channel.take() {
            channel
                .eof()
                .await
                .map_err(|e| anyhow!("Failed to send EOF: {}", e))?;
        }

        self.handle
            .disconnect(Disconnect::ByApplication, "", "English")
            .await
            .map_err(|e| anyhow!("Failed to disconnect: {}", e))?;

        Ok(())
    }
}
