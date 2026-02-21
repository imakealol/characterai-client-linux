pub mod characterai;
pub mod secrets;

#[derive(Clone, Default)]
pub struct AppServices {
    pub secrets: secrets::SecretsService,
    pub cai: characterai::CharacterAiService,
}

impl AppServices {
    pub fn new() -> Self {
        Self::default()
    }
}
