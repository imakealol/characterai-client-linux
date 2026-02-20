pub mod characterai;
pub mod secrets;

#[derive(Clone)]
pub struct AppServices {
    pub secrets: secrets::SecretsService,
    pub cai: characterai::CharacterAiService,
}

impl AppServices {
    pub fn new() -> Self {
        Self {
            secrets: secrets::SecretsService::new(),
            cai: characterai::CharacterAiService::new(),
        }
    }
}
