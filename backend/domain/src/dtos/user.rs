use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Clone)]
pub struct UserDTO {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub birthday: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserDTO {
    pub fn new(dto: CreateUserDTO) -> Self {
        Self {
            user_id: dto.user_id,
            username: dto.username,
            email: dto.email,
            birthday: dto.birthday,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct CreateUserDTO {
    pub user_id: Uuid,
    #[validate(length(min = 3, max = 24))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub birthday: NaiveDate,
}
