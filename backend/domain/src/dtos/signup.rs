use chrono::NaiveDate;
use email_verification_code::EmailVerificationCode;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct SignupEmailDTO {
    pub user_id: Uuid,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 42))]
    pub username: String,
    #[validate(length(min = 8, max = 256))]
    pub password: String,
    pub birthday: NaiveDate,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct EmailVerificationDTO {
    #[validate(email)]
    pub email: String,

    pub code: EmailVerificationCode,
}
