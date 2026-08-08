use async_trait::async_trait;

use crate::domain::chats::dtos::{chat_request::ChatRequest, response_message::ResponseMessage};
// use std::pin::Pin;

///Send -> Values can be sent to another thread with no major issues;
///Box -> Moves the value to Heap and holds its reference
///Pin -> This memory value should not be moved to another memory address.
#[async_trait]
// pub trait AiService: Send + Sync {
//     async fn complete(
//         &self,
//         request: AiRequest,
//     ) -> Result<AiResponse, AiServiceError>;
// }
// async_trait does all of that by himself
#[async_trait]
pub trait AiService: Send + Sync {
    async fn complete(&self, request: ChatRequest) -> Result<ResponseMessage, AiServiceError>;
}

///todo: take it somewhere else
pub struct AiServiceError {
    pub content: String,
}

impl AiServiceError {
    pub fn new(error_message: String) -> Self {
        return Self {
            content: error_message,
        };
    }
}
