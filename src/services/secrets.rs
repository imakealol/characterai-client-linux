use anyhow::Result;
use keyring::Entry;

const SERVICE: &str = "characterai-client-linux";
const ACCOUNT: &str = "auth_token";

/// Stores the CharacterAI auth token in the system keychain.
/// On KDE/Wayland this maps to KWallet via org.freedesktop.secrets.
#[derive(Clone, Default)]
pub struct SecretsService;

impl SecretsService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_auth_token(&self) -> Result<Option<String>> {
        let entry = Entry::new(SERVICE, ACCOUNT)?;
        match entry.get_password() {
            Ok(p) => Ok(Some(p)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn set_auth_token(&self, value: &str) -> Result<()> {
        let entry = Entry::new(SERVICE, ACCOUNT)?;
        entry.set_password(value)?;
        Ok(())
    }

    pub fn clear_auth_token(&self) -> Result<()> {
        let entry = Entry::new(SERVICE, ACCOUNT)?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
