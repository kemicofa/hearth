use async_trait::async_trait;
use errors::HearthError;

#[async_trait]
pub trait Feature<Input, Output>: Send + Sync {
    async fn execute(&self, input: Input) -> Result<Output, HearthError>;
}
