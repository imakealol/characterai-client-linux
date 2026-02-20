use anyhow::Result;

/// Placeholder secrets layer.
/// Next step: implement org.freedesktop.secrets (Secret Service),
/// which on KDE typically maps to KWallet.
#[derive(Clone, Default)]
pub struct SecretsService;

impl SecretsService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_auth_blob(&self) -> Result<Option<String>> {
        Ok(None)
    }

    pub async fn set_auth_blob(&self, _value: String) -> Result<()> {
        Ok(())
    }

    pub async fn clear_auth_blob(&self) -> Result<()> {
        Ok(())
    }
}
