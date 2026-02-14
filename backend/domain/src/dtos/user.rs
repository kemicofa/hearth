use chrono::{ DateTime, NaiveDate, Utc };
use serde::Deserialize;
use uuid::{ Uuid };
use validator::Validate;

#[derive(Debug, Validate, Clone)]
pub struct UserDTO {
    pub user_id: Uuid,
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
    #[validate(length(min = 3, max = 24))]
    pub username: String,
    pub birthday: NaiveDate,
    #[validate(email)]
    pub email: String,
}

impl CreateUserDTO {
    pub fn matches_email(&self, email: &String) -> bool {
        self.email == *email
    }
}
