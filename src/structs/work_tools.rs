use crate::structs::chat_request::ChatRequest;

//TODO: this needs to be dynamically set when configuring LLM definition
pub struct WorkTools {
    http_client: reqwest::Client,
    base_url: String,
    model: String,
}

impl WorkTools {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: base_url.into(),
            model: model.into(),
        }
    }

    pub async fn send_message(&self, body: &ChatRequest) -> Result<String, reqwest::Error> {
        let url = format!("{}/chat", self.base_url);
        let response = self.http_client.post(url).json(&body).send().await?;

        let content = response.text().await?;
        Ok(content)
    }
}
