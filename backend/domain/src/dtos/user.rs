use chrono::{ DateTime, NaiveDate, Utc };
use serde::Deserialize;
use uuid::{ Uuid, uuid };
use validator::Validate;

#[derive(Debug, Validate, Clone)]
pub struct UserDTO {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub birthday: NaiveDate,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserDTO {
    pub fn new(dto: CreateUserDTO) -> Self {
        Self {
            user_id: dto.user_id,
            first_name: dto.first_name,
            last_name: dto.last_name,
            username: dto.username,
            birthday: dto.birthday,
            email: dto.email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct CreateUserDTO {
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    #[validate(length(min = 3, max = 24))]
    pub username: String,
    pub birthday: NaiveDate,
    #[validate(email)]
    pub email: String,
}
