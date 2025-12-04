use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Group represents a folder for organizing SSH connections
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Identity represents an encrypted SSH key or password
#[derive(Debug, Clone, FromRow)]
pub struct Identity {
    pub id: String,
    pub name: String,
    pub encrypted_data: Vec<u8>,
    pub created_at: String,
    pub updated_at: String,
}

/// IdentityData represents the decrypted identity (in-memory only)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IdentityData {
    #[serde(rename = "ssh_key")]
    SshKey {
        private_key: String,
        passphrase: Option<String>,
    },
    #[serde(rename = "password")]
    Password { password: String },
}

/// Host represents an SSH connection configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Host {
    pub id: String,
    pub group_id: Option<String>,
    pub identity_id: Option<String>,
    pub name: String,
    pub hostname: String,
    pub port: i64,
    pub username: String,
    pub tags: Option<String>, // JSON array
    pub created_at: String,
    pub updated_at: String,
}

impl Host {
    /// Parse tags from JSON string
    pub fn get_tags(&self) -> Vec<String> {
        self.tags
            .as_ref()
            .and_then(|t| serde_json::from_str(t).ok())
            .unwrap_or_default()
    }

    /// Set tags as JSON string
    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = serde_json::to_string(&tags).ok();
    }
}
