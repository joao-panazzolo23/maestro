use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize)]
pub struct ChatMessage {}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}
