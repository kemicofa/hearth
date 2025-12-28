#[cfg(test)]
pub mod test_utils {
    use std::{ collections::HashMap, sync::{ Arc, Mutex } };

    use crate::{
        dtos::user::{ CreateUserDTO, UserDTO },
        error_codes::USER_NOT_FOUND_ERROR_CODE,
        repositories::{
            email_sender_repository::EmailSenderRepository,
            email_verifications_repository::EmailVerificationRepository,
            tmp_users_repository::TemporaryUsersRepository,
            users_repository::UsersRepository,
        },
    };
    use async_trait::async_trait;
    use errors::ZwitterError;
    use uuid::Uuid;
    use email_verification_code::EmailVerificationCode;

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
        ) -> Result<(), ZwitterError> {
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
        ) -> Result<(), ZwitterError> {
            self.map.lock().unwrap().insert(email.clone(), code.clone());
            Ok(())
        }

        async fn code_matches(
            &self,
            email: &String,
            code: &EmailVerificationCode
        ) -> Result<bool, ZwitterError> {
            let map = self.map.lock().unwrap();
            let stored_code = map.get(email);
            Ok(stored_code.is_some_and(|v| v == code))
        }
    }

    pub struct InMemoryTmpUsersRepository {
        map: Arc<Mutex<HashMap<Uuid, String>>>,
    }

    impl Default for InMemoryTmpUsersRepository {
        fn default() -> Self {
            Self {
                map: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    impl InMemoryTmpUsersRepository {
        pub fn from_existing_tmp_user(tmp_user_id: Uuid, email: String) -> Self {
            Self {
                map: Arc::new(Mutex::new(HashMap::from([(tmp_user_id, email)]))),
            }
        }
    }

    #[async_trait]
    impl TemporaryUsersRepository for InMemoryTmpUsersRepository {
        async fn store(&self, tmp_user_id: &Uuid, email: &String) -> Result<(), ZwitterError> {
            let mut map = self.map.lock().unwrap();
            map.insert(tmp_user_id.clone(), email.clone());
            Ok(())
        }
        async fn get_email(&self, tmp_user_id: &Uuid) -> Result<String, ZwitterError> {
            let map = self.map.lock().unwrap();
            let opt = map.get(&tmp_user_id);

            if opt.is_none() {
                return Err(ZwitterError::not_found("TEMPORARY_USER_NOT_FOUND".into()));
            }
            Ok(opt.unwrap().clone())
        }
    }

    #[derive(Debug)]
    pub struct InMemoryUserRepository {
        users: Arc<Mutex<HashMap<String, UserDTO>>>,
    }

    impl Default for InMemoryUserRepository {
        fn default() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::default())),
            }
        }
    }

    impl InMemoryUserRepository {
        pub fn from_existing_user(dto: CreateUserDTO) -> Self {
            let users: HashMap<String, UserDTO> = HashMap::from([
                (dto.user_id.to_string(), UserDTO::new(dto)),
            ]);
            Self {
                users: Arc::new(Mutex::new(users)),
            }
        }
    }

    #[async_trait]
    impl UsersRepository for InMemoryUserRepository {
        async fn create(&self, dto: CreateUserDTO) -> Result<(), ZwitterError> {
            self.users.lock().unwrap().insert(dto.user_id.to_string(), UserDTO::new(dto));
            Ok(())
        }

        async fn get(&self, user_id: String) -> Result<UserDTO, ZwitterError> {
            match self.users.lock().unwrap().get(&user_id) {
                Some(user_dto) => Ok(user_dto.clone()),
                None => Err(ZwitterError::not_found(USER_NOT_FOUND_ERROR_CODE.into())),
            }
        }

        async fn exists(&self, email: &String) -> Result<bool, ZwitterError> {
            let users = self.users.lock().unwrap();

            let opt = users.iter().find(|user| user.1.email == *email);

            Ok(opt.is_some())
        }
    }
}
