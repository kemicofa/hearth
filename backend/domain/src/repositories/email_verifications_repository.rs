use async_trait::async_trait;
use email_verification_code::EmailVerificationCode;
use errors::HearthError;

#[async_trait]
pub trait EmailVerificationRepository: Send + Sync {
    async fn store(&self, email: &String, code: &EmailVerificationCode) -> Result<(), HearthError>;

    async fn code_matches(
        &self,
        email: &String,
        code: &EmailVerificationCode,
    ) -> Result<bool, HearthError>;
}
