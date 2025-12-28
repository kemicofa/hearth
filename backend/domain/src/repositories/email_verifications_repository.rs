use async_trait::async_trait;
use email_verification_code::EmailVerificationCode;
use errors::ZwitterError;

#[async_trait]
pub trait EmailVerificationRepository: Send + Sync {
    async fn store(&self, email: &String, code: &EmailVerificationCode) -> Result<(), ZwitterError>;

    async fn code_matches(
        &self,
        email: &String,
        code: &EmailVerificationCode
    ) -> Result<bool, ZwitterError>;
}
