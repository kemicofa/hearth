use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    dtos::{ auth::CredentialsDTO, user::{ CreateUserDTO, UserDTO } },
    repositories::users_repository::UsersRepository,
};
use errors::HearthError;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::Set,
    DatabaseConnection,
    TransactionError,
    TransactionTrait,
};

use crate::database::entities::{ credentials, users };

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
        credentials_dto: CredentialsDTO
    ) -> Result<(), HearthError> {
        self.connection
            .transaction::<_, (), HearthError>(|transaction| {
                Box::pin(async move {
                    (users::ActiveModel {
                        username: Set(dto.username),
                        email: Set(dto.email),
                        id: Set(dto.user_id),
                        birthday: Set(dto.birthday),
                        ..Default::default()
                    })
                        .save(transaction).await
                        .map_err(|e|
                            HearthError::unexpected("CREATE_USER_ERROR".into(), Some(e.to_string()))
                        )?;

                    (credentials::ActiveModel {
                        id: Set(uuid::Uuid::new_v4()),
                        user_id: Set(credentials_dto.user_id.to_string()),
                        password_hash: Set(credentials_dto.password_hash),
                        ..Default::default()
                    })
                        .save(transaction).await
                        .map_err(|e|
                            HearthError::unexpected(
                                "INSERT_CREDENTIALS_ERROR".into(),
                                Some(e.to_string())
                            )
                        )?;

                    Ok(())
                })
            }).await
            .map_err(|e| {
                match e {
                    TransactionError::Transaction(err) => err,
                    TransactionError::Connection(err) =>
                        HearthError::unexpected(
                            "DATABASE_CONNECTION_ERROR".into(),
                            Some(err.to_string())
                        ),
                }
            })?;

        Ok(())
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

#[cfg(test)]
mod tests {}
