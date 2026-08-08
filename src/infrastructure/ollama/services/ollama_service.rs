use async_trait::async_trait;
use reqwest::Client;

use crate::domain::{
    ai_models::services::ai_model_service::{AiService, AiServiceError},
    chats::dtos::{chat_request::ChatRequest, response_message::ResponseMessage},
};

//todo: implement "ai model service" trait as a service here
pub struct OllamaService {
    client: reqwest::Client,
    base_url: String,
}

impl OllamaService {
    pub fn new(client: Client, base_url: String) -> Self {
        return Self { client, base_url };
    }
}

#[async_trait]
impl AiService for OllamaService {
    async fn complete(&self, request: ChatRequest) -> Result<ResponseMessage, AiServiceError> {
        let response = self
            .client
            .post(self.base_url.clone())
            .json(&request)
            .send()
            .await
            .map_err(|error| AiServiceError::new(error.to_string()))?;

        let message = response
            .json::<ResponseMessage>()
            .await
            .map_err(|error| AiServiceError::new(error.to_string()))?;

        return Ok(message);
    }
}
