use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const BASE: &str = "https://neo.character.ai";

#[derive(Debug, Serialize)]
struct CreateChatRequest<'a> {
    character_id: &'a str,
}

#[derive(Debug, Deserialize)]
struct CreateChatResponse {
    chat: ChatInfo,
}

#[derive(Debug, Deserialize)]
struct ChatInfo {
    chat_id: String,
}

#[derive(Debug, Serialize)]
struct TurnCreateRequest<'a> {
    chat_id: &'a str,
    num_candidates: u8,
    turn: TurnPayload<'a>,
}

#[derive(Debug, Serialize)]
struct TurnPayload<'a> {
    author: TurnAuthor,
    candidates: Vec<TurnCandidate<'a>>,
}

#[derive(Debug, Serialize)]
struct TurnAuthor {
    author_id: String,
}

#[derive(Debug, Serialize)]
struct TurnCandidate<'a> {
    raw_content: &'a str,
}

#[derive(Debug, Deserialize)]
struct TurnChunk {
    turn: Option<TurnResult>,
}

#[derive(Debug, Deserialize)]
struct TurnResult {
    candidates: Option<Vec<CandidateResult>>,
}

#[derive(Debug, Deserialize)]
struct CandidateResult {
    raw_content: Option<String>,
}

/// Communicates with the Character.AI v1 REST API.
#[derive(Clone, Default)]
pub struct CharacterAiService;

impl CharacterAiService {
    pub fn new() -> Self {
        Self
    }

    /// Create a new chat session with the given character and return its chat_id.
    pub fn create_chat(&self, token: &str, character_id: &str) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(format!("{BASE}/api/v1/chat/create/"))
            .header("Authorization", format!("Token {token}"))
            .header("Content-Type", "application/json")
            .json(&CreateChatRequest { character_id })
            .send()
            .context("HTTP request to create chat failed")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().unwrap_or_default();
            anyhow::bail!("create_chat HTTP {status}: {body}");
        }

        let data: CreateChatResponse = resp.json().context("failed to parse create_chat response")?;
        Ok(data.chat.chat_id)
    }

    /// Send a message and return the assistant's reply text.
    pub fn send_message(
        &self,
        token: &str,
        chat_id: &str,
        author_id: &str,
        text: &str,
    ) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let body = TurnCreateRequest {
            chat_id,
            num_candidates: 1,
            turn: TurnPayload {
                author: TurnAuthor { author_id: author_id.to_owned() },
                candidates: vec![TurnCandidate { raw_content: text }],
            },
        };
        let resp = client
            .post(format!("{BASE}/api/v1/chat/turn/create/"))
            .header("Authorization", format!("Token {token}"))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .context("HTTP request to send message failed")?;

        let status = resp.status();
        if !status.is_success() {
            let body_text = resp.text().unwrap_or_default();
            anyhow::bail!("send_message HTTP {status}: {body_text}");
        }

        // The response is an SSE stream; collect lines that start with "data: "
        // and pick the last non-empty turn with a candidate.
        let raw = resp.text().context("failed to read send_message response body")?;
        let mut last_content = String::new();
        for line in raw.lines() {
            let data = if let Some(rest) = line.strip_prefix("data: ") {
                rest
            } else {
                continue;
            };
            if let Ok(chunk) = serde_json::from_str::<TurnChunk>(data) {
                if let Some(turn) = chunk.turn {
                    if let Some(candidates) = turn.candidates {
                        if let Some(candidate) = candidates.into_iter().next() {
                            if let Some(content) = candidate.raw_content {
                                if !content.is_empty() {
                                    last_content = content;
                                }
                            }
                        }
                    }
                }
            }
        }

        if last_content.is_empty() {
            anyhow::bail!("No response received from CharacterAI");
        }
        Ok(last_content)
    }
}
