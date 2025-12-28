use async_trait::async_trait;
use domain::repositories::email_sender_repository::EmailSenderRepository;
use email_verification_code::EmailVerificationCode;
use errors::ZwitterError;

pub struct EmailSenderGateway {}

#[async_trait]
impl EmailSenderRepository for EmailSenderGateway {
    async fn send_verify_email(
        &self,
        email: &String,
        code: &EmailVerificationCode
    ) -> Result<(), ZwitterError> {
        todo!();
    }
}
