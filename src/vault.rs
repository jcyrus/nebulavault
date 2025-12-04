use age::secrecy::{ExposeSecret, Secret};
use anyhow::{Context, Result};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, ParamsBuilder,
};
use std::io::{Read, Write};

use crate::models::IdentityData;

/// Vault handles encryption and decryption of sensitive data
pub struct Vault {
    master_key: Option<Secret<Vec<u8>>>,
}

impl std::fmt::Debug for Vault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vault")
            .field("master_key", &"<redacted>")
            .finish()
    }
}

impl Vault {
    /// Create a new Vault instance
    pub fn new() -> Self {
        Self { master_key: None }
    }

    /// Derive a key from the master password using Argon2id
    /// Parameters follow OWASP recommendations: m=19MB, t=2, p=1
    /// 
    /// IMPORTANT: Uses a deterministic salt derived from the password itself
    /// to ensure the same password always produces the same encryption key.
    /// This is necessary for encrypting/decrypting data across sessions.
    pub fn derive_key(&mut self, password: &str) -> Result<()> {
        // Use a deterministic salt based on the password
        // This ensures the same password always produces the same key
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"nebulavault-salt-v1");
        hasher.update(password.as_bytes());
        let salt_bytes = hasher.finalize();
        
        // Convert to SaltString format (base64)
        let salt = SaltString::encode_b64(&salt_bytes[..16])
            .map_err(|e| anyhow::anyhow!("Failed to encode salt: {:?}", e))?;

        // Configure Argon2 parameters (OWASP recommendations)
        let params = ParamsBuilder::new()
            .m_cost(19 * 1024) // 19 MB
            .t_cost(2) // 2 iterations
            .p_cost(1) // 1 parallelism
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build Argon2 params: {:?}", e))?;

        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            params,
        );

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {:?}", e))?;

        // Extract the hash bytes as the encryption key
        let hash_bytes = password_hash
            .hash
            .context("No hash in password hash")?
            .as_bytes()
            .to_vec();

        self.master_key = Some(Secret::new(hash_bytes));
        Ok(())
    }

    /// Encrypt identity data using age
    pub fn encrypt_identity(&self, data: &IdentityData) -> Result<Vec<u8>> {
        let key = self
            .master_key
            .as_ref()
            .context("Master key not derived")?;

        // Serialize the identity data
        let json = serde_json::to_vec(data).context("Failed to serialize identity data")?;

        // Create age encryptor with passphrase
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(
            String::from_utf8_lossy(key.expose_secret()).to_string(),
        ));

        let mut encrypted = Vec::new();
        let mut writer = encryptor
            .wrap_output(&mut encrypted)
            .context("Failed to create age encryptor")?;

        writer
            .write_all(&json)
            .context("Failed to write encrypted data")?;
        writer
            .finish()
            .context("Failed to finalize encryption")?;

        Ok(encrypted)
    }

    /// Decrypt identity data using age
    pub fn decrypt_identity(&self, encrypted_data: &[u8]) -> Result<IdentityData> {
        let key = self
            .master_key
            .as_ref()
            .context("Master key not derived")?;

        // Create age decryptor with passphrase
        let decryptor = match age::Decryptor::new(encrypted_data)
            .context("Failed to create age decryptor")?
        {
            age::Decryptor::Passphrase(d) => d,
            _ => anyhow::bail!("Unexpected decryptor type"),
        };

        let mut decrypted = Vec::new();
        let mut reader = decryptor
            .decrypt(
                &age::secrecy::Secret::new(
                    String::from_utf8_lossy(key.expose_secret()).to_string(),
                ),
                None,
            )
            .context("Failed to decrypt (wrong password?)")?;

        reader
            .read_to_end(&mut decrypted)
            .context("Failed to read decrypted data")?;

        // Deserialize the identity data
        let data: IdentityData =
            serde_json::from_slice(&decrypted).context("Failed to deserialize identity data")?;

        Ok(data)
    }

    /// Export the entire database to an encrypted blob
    pub fn export_to_blob(&self, db_path: &str) -> Result<Vec<u8>> {
        let key = self
            .master_key
            .as_ref()
            .context("Master key not derived")?;

        // Read the database file
        let db_data = std::fs::read(db_path).context("Failed to read database file")?;

        // Compress the data
        let compressed = compress_data(&db_data)?;

        // Encrypt the compressed data
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(
            String::from_utf8_lossy(key.expose_secret()).to_string(),
        ));

        let mut encrypted = Vec::new();
        let mut writer = encryptor
            .wrap_output(&mut encrypted)
            .context("Failed to create age encryptor")?;

        writer
            .write_all(&compressed)
            .context("Failed to write encrypted data")?;
        writer
            .finish()
            .context("Failed to finalize encryption")?;

        Ok(encrypted)
    }

    /// Import database from an encrypted blob
    pub fn import_from_blob(&self, blob: &[u8], db_path: &str) -> Result<()> {
        let key = self
            .master_key
            .as_ref()
            .context("Master key not derived")?;

        // Decrypt the blob
        let decryptor =
            match age::Decryptor::new(blob).context("Failed to create age decryptor")? {
                age::Decryptor::Passphrase(d) => d,
                _ => anyhow::bail!("Unexpected decryptor type"),
            };

        let mut decrypted = Vec::new();
        let mut reader = decryptor
            .decrypt(
                &age::secrecy::Secret::new(
                    String::from_utf8_lossy(key.expose_secret()).to_string(),
                ),
                None,
            )
            .context("Failed to decrypt (wrong password?)")?;

        reader
            .read_to_end(&mut decrypted)
            .context("Failed to read decrypted data")?;

        // Decompress the data
        let db_data = decompress_data(&decrypted)?;

        // Write to database file
        std::fs::write(db_path, db_data).context("Failed to write database file")?;

        Ok(())
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}

/// Compress data using flate2 (gzip)
fn compress_data(data: &[u8]) -> Result<Vec<u8>> {
    use flate2::write::GzEncoder;
    use flate2::Compression;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(data)
        .context("Failed to compress data")?;
    encoder.finish().context("Failed to finalize compression")
}

/// Decompress data using flate2 (gzip)
fn decompress_data(data: &[u8]) -> Result<Vec<u8>> {
    use flate2::read::GzDecoder;

    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .context("Failed to decompress data")?;
    Ok(decompressed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::IdentityData;

    #[test]
    fn test_encrypt_decrypt_identity() {
        let mut vault = Vault::new();
        vault.derive_key("test_password").unwrap();

        let identity = IdentityData::Password {
            password: "secret123".to_string(),
        };

        let encrypted = vault.encrypt_identity(&identity).unwrap();
        let decrypted = vault.decrypt_identity(&encrypted).unwrap();

        match decrypted {
            IdentityData::Password { password } => {
                assert_eq!(password, "secret123");
            }
            _ => panic!("Wrong identity type"),
        }
    }
}
