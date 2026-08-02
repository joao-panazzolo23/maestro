use serde::{Deserialize, Serialize};

use crate::structs::chat_message::ChatMessage;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}
