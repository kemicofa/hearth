use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    dtos::{
        auth::CredentialsDTO,
        user::{CreateUserDTO, UserDTO},
    },
    repositories::users_repository::UsersRepository,
};
use errors::HearthError;
use sea_orm::DatabaseConnection;

pub struct UsersRepositoryPostgres {
    connection: Arc<DatabaseConnection>,
}

impl UsersRepositoryPostgres {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl UsersRepository for UsersRepositoryPostgres {
    async fn create(
        &self,
        dto: CreateUserDTO,
        credentials_dto: CredentialsDTO,
    ) -> Result<(), HearthError> {
        todo!();
    }

    async fn get(&self, user_id: String) -> Result<UserDTO, HearthError> {
        todo!();
    }

    async fn email_exists(&self, email: &String) -> Result<bool, HearthError> {
        todo!();
    }

    async fn username_exists(&self, username: &String) -> Result<bool, HearthError> {
        todo!();
    }
}
