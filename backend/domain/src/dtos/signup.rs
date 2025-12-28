use email_verification_code::EmailVerificationCode;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct SignupEmailDTO {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct EmailVerificationDTO {
    #[validate(email)]
    pub email: String,

    pub code: EmailVerificationCode,
}
