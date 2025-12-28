use async_trait::async_trait;
use errors::ZwitterError;

use crate::dtos::user::{ CreateUserDTO, UserDTO };

#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn create(&self, dto: CreateUserDTO) -> Result<(), ZwitterError>;

    async fn get(&self, user_id: String) -> Result<UserDTO, ZwitterError>;

    async fn exists(&self, email: &String) -> Result<bool, ZwitterError>;
}
