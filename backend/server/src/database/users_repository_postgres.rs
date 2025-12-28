use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    dtos::user::{ CreateUserDTO, UserDTO },
    repositories::users_repository::UsersRepository,
};
use errors::ZwitterError;
use sea_orm::DatabaseConnection;

pub struct UsersRepositoryPostgres {
    connection: Arc<DatabaseConnection>,
}

impl UsersRepositoryPostgres {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self {
            connection,
        }
    }
}

#[async_trait]
impl UsersRepository for UsersRepositoryPostgres {
    async fn create(&self, dto: CreateUserDTO) -> Result<(), ZwitterError> {
        todo!();
    }

    async fn get(&self, user_id: String) -> Result<UserDTO, ZwitterError> {
        todo!();
    }

    async fn exists(&self, email: &String) -> Result<bool, ZwitterError> {
        todo!();
    }
}
