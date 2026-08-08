use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    content: String,
}
