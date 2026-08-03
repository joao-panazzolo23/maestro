use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}
