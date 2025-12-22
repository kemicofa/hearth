#[cfg(test)]
pub mod test_utils {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use crate::{
        dtos::user::{CreateUserDTO, UserDTO},
        error_codes::USER_NOT_FOUND_ERROR_CODE,
        features::create_user::CreateUser,
        repositories::users_repository::UsersRepository,
    };
    use async_trait::async_trait;
    use errors::ZwitterError;
    use macros::barc;

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

    #[async_trait]
    impl UsersRepository for InMemoryUserRepository {
        async fn create(&self, dto: CreateUserDTO) -> Result<(), ZwitterError> {
            self.users
                .lock()
                .unwrap()
                .insert(dto.user_id.to_string(), UserDTO::new(dto));
            Ok(())
        }

        async fn get(&self, user_id: String) -> Result<UserDTO, ZwitterError> {
            match self.users.lock().unwrap().get(&user_id) {
                Some(user_dto) => Ok(user_dto.clone()),
                None => Err(ZwitterError::not_found(USER_NOT_FOUND_ERROR_CODE.into())),
            }
        }
    }

    impl Default for CreateUser {
        fn default() -> Self {
            Self {
                users_repository: barc!(InMemoryUserRepository::default()),
            }
        }
    }
}
