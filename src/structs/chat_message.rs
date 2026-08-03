use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize)]
pub struct ChatMessage {
    role: String,
    content: String,
}
