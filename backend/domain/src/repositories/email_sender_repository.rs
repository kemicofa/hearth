use async_trait::async_trait;
use email_verification_code::EmailVerificationCode;
use errors::ZwitterError;

#[async_trait]
pub trait EmailSenderRepository: Send + Sync {
    async fn send_verify_email(
        &self,
        email: &String,
        code: &EmailVerificationCode
    ) -> Result<(), ZwitterError>;
}
