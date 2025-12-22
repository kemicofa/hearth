use async_trait::async_trait;
use errors::ZwitterError;

#[async_trait]
pub trait Feature<Input, Output>: Send + Sync {
    async fn execute(&self, input: Input) -> Result<Output, ZwitterError>;
}
