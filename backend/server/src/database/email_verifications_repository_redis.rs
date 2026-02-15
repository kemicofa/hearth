use std::sync::Arc;

use async_trait::async_trait;
use domain::repositories::email_verifications_repository::EmailVerificationRepository;
use email_verification_code::EmailVerificationCode;
use errors::HearthError;
use redis::{AsyncCommands, Client, RedisError};

pub struct EmailVerificationsRepositoryRedis {
    client: Arc<Client>,
    ttl: u64,
}

impl EmailVerificationsRepositoryRedis {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            ttl: 60 * 60, // TTL set to expire after an hour
        }
    }
}

#[async_trait]
impl EmailVerificationRepository for EmailVerificationsRepositoryRedis {
    async fn store(&self, email: &String, code: &EmailVerificationCode) -> Result<(), HearthError> {
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                HearthError::unexpected("EVR_STORE_ASYNC_CON".into(), Some(e.to_string()))
            })?;

        match con
            .set_ex::<&String, &String, u64>(email, &code.code, self.ttl)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(HearthError::unexpected(
                "EVR_STORE".into(),
                Some(e.to_string()),
            )),
        }
    }

    async fn code_matches(
        &self,
        email: &String,
        evc: &EmailVerificationCode,
    ) -> Result<bool, HearthError> {
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                HearthError::unexpected("EVR_CODE_MATCHES_ASYNC_CON".into(), Some(e.to_string()))
            })?;

        let code = con.get::<&String, String>(email).await.map_err(|e| {
            RedisError::code(&e);
            HearthError::unexpected("EVR_CODE_MATCHES_ASYNG_GET".into(), Some(e.to_string()))
        })?;

        Ok(evc.code == code)
    }
}
