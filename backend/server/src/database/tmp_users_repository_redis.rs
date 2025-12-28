use async_trait::async_trait;
use domain::repositories::tmp_users_repository::TemporaryUsersRepository;
use errors::ZwitterError;
use uuid::Uuid;

pub struct TemporaryUsersRepositoryRedis {}

impl TemporaryUsersRepositoryRedis {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl TemporaryUsersRepository for TemporaryUsersRepositoryRedis {
    async fn store(&self, tmp_user_id: &Uuid, email: &String) -> Result<(), ZwitterError> {
        todo!();
    }
    async fn get_email(&self, tmp_user_id: &Uuid) -> Result<String, ZwitterError> {
        todo!();
    }
}
