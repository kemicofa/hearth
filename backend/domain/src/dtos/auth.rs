use uuid::Uuid;

#[derive(Debug)]
pub struct CredentialsDTO {
    pub user_id: Uuid,
    pub password: String,
}
