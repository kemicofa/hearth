use async_trait::async_trait;
use errors::HearthError;

use crate::dtos::{
    auth::CredentialsDTO,
    user::{CreateUserDTO, UserDTO},
};

#[async_trait]
pub trait UsersRepository: Send + Sync {
    /// standard signup with email/password combination
    async fn create(
        &self,
        create_user_dto: CreateUserDTO,
        credentials_dto: CredentialsDTO,
    ) -> Result<(), HearthError>;
    async fn get(&self, user_id: String) -> Result<UserDTO, HearthError>;
    async fn email_exists(&self, email: &String) -> Result<bool, HearthError>;
    async fn username_exists(&self, username: &String) -> Result<bool, HearthError>;
}
