use async_trait::async_trait;

use crate::domain::behaviors::entities::behavior::Behavior;
///Todo: this could return an explicit type of return.
#[async_trait]
pub trait BehaviorRepository: Send + Sync {
    async fn create(&self, behavior: Behavior) -> Result<(), RepositoryError>;
    async fn delete(&self, behavior: Behavior) -> Result<(), RepositoryError>;
    async fn update(&self, behavior: Behavior) -> Result<(), RepositoryError>;
    async fn get(&self, id: String) -> Result<Behavior, RepositoryError>;
}

///todo: move this away, maybe shared, whatever
pub struct RepositoryError {
    content: String,
    inner_ex: String,
}

impl RepositoryError {
    pub fn new(content: String, inner_ex: String) -> Self {
        return Self { content, inner_ex };
    }
}
