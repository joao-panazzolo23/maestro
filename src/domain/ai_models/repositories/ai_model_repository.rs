use async_trait::async_trait;

use crate::domain::{
    ai_models::entities::ai_model::AiModel,
    behaviors::repositories::behavior_repository::RepositoryError,
};

#[async_trait]
pub trait AiModelRepository: Send + Sync {
    async fn create(&self, ai_model: AiModel) -> Result<(), RepositoryError>;
    async fn delete(&self, ai_model: AiModel) -> Result<(), RepositoryError>;
    async fn update(&self, ai_model: AiModel) -> Result<(), RepositoryError>;
    async fn get(&self, id: String) -> Result<AiModel, RepositoryError>;
}
