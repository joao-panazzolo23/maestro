use serde::de::DeserializeOwned;

use crate::domain::chats::dtos::chat_request::ChatRequest;

//TODO: this needs to be dynamically set when configuring LLM definition
#[derive(Debug, Clone)]
pub struct WorkTools {
    http_client: reqwest::Client,
    base_url: String,
    model: String,
}

///TODO: i dont think thats a real deal when it comes to decoupling. structs still being concrete implementations,
/// maybe using its own "interface" as a trait within domain layer and implementing at infrastructure level
impl WorkTools {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> WorkTools {
        return WorkTools {
            http_client: reqwest::Client::new(),
            base_url: base_url.into(),
            model: model.into(),
        };
    }

    pub async fn send_message<T: DeserializeOwned>(
        &self,
        body: &ChatRequest,
    ) -> Result<T, reqwest::Error> {
        let url = format!("{}/chat", self.base_url);
        let response: T = self
            .http_client
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        return Ok(response);
    }
}
