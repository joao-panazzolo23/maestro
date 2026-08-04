use serde::{Deserialize, Serialize};

use crate::structs::chat_message::ChatMessage;

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}
