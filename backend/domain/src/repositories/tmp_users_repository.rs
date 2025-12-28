use async_trait::async_trait;
use errors::ZwitterError;
use uuid::Uuid;

#[async_trait]
pub trait TemporaryUsersRepository: Send + Sync {
    async fn store(&self, tmp_user_id: &Uuid, email: &String) -> Result<(), ZwitterError>;
    async fn get_email(&self, tmp_user_id: &Uuid) -> Result<String, ZwitterError>;
}
