use serde::Serialize;

use crate::domain::chats::dtos::chat_message::ChatMessage;

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}
