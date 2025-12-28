use std::sync::Arc;

use async_trait::async_trait;
use domain::repositories::email_verifications_repository::EmailVerificationRepository;
use email_verification_code::EmailVerificationCode;
use errors::ZwitterError;
use redis::Client;

pub struct EmailVerificationsRepositoryRedis {
    client: Arc<Client>,
}

impl EmailVerificationsRepositoryRedis {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
        }
    }
}

#[async_trait]
impl EmailVerificationRepository for EmailVerificationsRepositoryRedis {
    async fn store(
        &self,
        email: &String,
        code: &EmailVerificationCode
    ) -> Result<(), ZwitterError> {
        todo!();
    }

    async fn code_matches(
        &self,
        email: &String,
        code: &EmailVerificationCode
    ) -> Result<bool, ZwitterError> {
        todo!();
    }
}
