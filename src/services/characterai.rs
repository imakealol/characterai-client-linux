use anyhow::Result;

/// Placeholder CharacterAI service.
/// Next step: spawn a Node bridge that wraps `realcoloride/node_characterai`
/// and talk via stdin/stdout JSON messages.
#[derive(Clone, Default)]
pub struct CharacterAiService;

impl CharacterAiService {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_message(&self, _chat_id: &str, _text: &str) -> Result<String> {
        Ok("Not implemented yet".to_string())
    }
}
