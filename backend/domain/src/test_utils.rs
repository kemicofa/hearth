#[cfg(test)]
pub mod test_utils {
    use std::{ collections::HashMap, sync::{ Arc, Mutex } };

    use crate::{
        dtos::{ auth::CredentialsDTO, user::{ CreateUserDTO, UserDTO } },
        error_codes::USER_NOT_FOUND_ERROR_CODE,
        repositories::{
            credentials_repository::CredentialsRepository,
            email_sender_repository::EmailSenderRepository,
            email_verifications_repository::EmailVerificationRepository,
            users_repository::UsersRepository,
        },
    };
    use async_trait::async_trait;
    use email_verification_code::EmailVerificationCode;
    use errors::HearthError;
    use uuid::Uuid;

    pub struct InMemoryEmailSenderRepository {}

    impl Default for InMemoryEmailSenderRepository {
        fn default() -> Self {
            Self {}
        }
    }

    #[async_trait]
    impl EmailSenderRepository for InMemoryEmailSenderRepository {
        async fn send_verify_email(
            &self,
            _email: &String,
            _code: &EmailVerificationCode
        ) -> Result<(), HearthError> {
            Ok(())
        }
    }

    pub struct InMemoryEmailVerificationRepository {
        map: Arc<Mutex<HashMap<String, EmailVerificationCode>>>,
    }

    impl InMemoryEmailVerificationRepository {
        pub fn from_email_and_code(email: String, code: EmailVerificationCode) -> Self {
            Self {
                map: Arc::new(Mutex::new(HashMap::from([(email, code)]))),
            }
        }
    }

    impl Default for InMemoryEmailVerificationRepository {
        fn default() -> Self {
            Self {
                map: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl EmailVerificationRepository for InMemoryEmailVerificationRepository {
        async fn store(
            &self,
            email: &String,
            code: &EmailVerificationCode
        ) -> Result<(), HearthError> {
            self.map.lock().unwrap().insert(email.clone(), code.clone());
            Ok(())
        }

        async fn code_matches(
            &self,
            email: &String,
            code: &EmailVerificationCode
        ) -> Result<bool, HearthError> {
            let map = self.map.lock().unwrap();
            let stored_code = map.get(email);
            Ok(stored_code.is_some_and(|v| v == code))
        }
    }

    #[derive(Debug)]
    pub struct InMemoryUserRepository {
        users: Arc<Mutex<HashMap<String, UserDTO>>>,
        credentials: Arc<Mutex<HashMap<String, String>>>,
    }

    impl Default for InMemoryUserRepository {
        fn default() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::default())),
                credentials: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    impl InMemoryUserRepository {
        pub fn from_existing_user(dto: CreateUserDTO, credentials_dto: CredentialsDTO) -> Self {
            let users: HashMap<String, UserDTO> = HashMap::from([
                (dto.user_id.to_string(), UserDTO::new(dto)),
            ]);

            let credentials: HashMap<String, String> = HashMap::from([
                (credentials_dto.user_id.to_string(), credentials_dto.password_hash.clone()),
            ]);
            Self {
                users: Arc::new(Mutex::new(users)),
                credentials: Arc::new(Mutex::new(credentials)),
            }
        }
    }

    #[async_trait]
    impl UsersRepository for InMemoryUserRepository {
        async fn create(
            &self,
            dto: CreateUserDTO,
            credentials_dto: CredentialsDTO
        ) -> Result<(), HearthError> {
            self.users.lock().unwrap().insert(dto.user_id.to_string(), UserDTO::new(dto));
            self.credentials
                .lock()
                .unwrap()
                .insert(credentials_dto.user_id.to_string(), credentials_dto.password_hash.clone());
            Ok(())
        }

        async fn get(&self, user_id: String) -> Result<UserDTO, HearthError> {
            match self.users.lock().unwrap().get(&user_id) {
                Some(user_dto) => Ok(user_dto.clone()),
                None => Err(HearthError::not_found(USER_NOT_FOUND_ERROR_CODE.into())),
            }
        }

        async fn username_exists(&self, username: &String) -> Result<bool, HearthError> {
            let users = self.users.lock().unwrap();

            let opt = users.iter().find(|user| user.1.username == *username);

            Ok(opt.is_some())
        }

        async fn email_exists(&self, email: &String) -> Result<bool, HearthError> {
            let users = self.users.lock().unwrap();

            let opt = users.iter().find(|user| user.1.email == *email);

            Ok(opt.is_some())
        }
    }
}
