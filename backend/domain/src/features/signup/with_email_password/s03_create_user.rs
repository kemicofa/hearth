use async_trait::async_trait;
use errors::ZwitterError;
use macros::BArc;
use validator::Validate;

use crate::{
    dtos::user::CreateUserDTO,
    features::feature::Feature,
    repositories::{
        tmp_users_repository::TemporaryUsersRepository,
        users_repository::UsersRepository,
    },
};

pub struct CreateUser {
    pub users_repository: BArc<dyn UsersRepository>,
    pub tmp_users_repository: BArc<dyn TemporaryUsersRepository>,
}

pub type CreateUserFeature = dyn Feature<CreateUserDTO, ()>;

#[async_trait]
impl Feature<CreateUserDTO, ()> for CreateUser {
    async fn execute(&self, input: CreateUserDTO) -> Result<(), errors::ZwitterError> {
        if let Err(e) = input.validate() {
            return Err(ZwitterError::Validation("CREATE_USER".into(), e));
        }

        let email = self.tmp_users_repository.get_email(&input.user_id).await?;

        if !input.matches_email(&email) {
            return Err(ZwitterError::Domain("CREATE_USER_EMAIL_MISMATCH".into()));
        }

        self.users_repository.create(input).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use errors::ZwitterError;
    use macros::{ BArc, barc };
    use uuid::Uuid;

    use crate::{
        dtos::user::CreateUserDTO,
        features::{ feature::Feature, signup::with_email_password::s03_create_user::CreateUser },
        repositories::{
            tmp_users_repository::TemporaryUsersRepository,
            users_repository::UsersRepository,
        },
        test_utils::test_utils::{ InMemoryTmpUsersRepository, InMemoryUserRepository },
    };

    impl CreateUser {
        fn new(dto: Option<CreateUserDTO>, tmp_user_data: Option<(Uuid, String)>) -> Self {
            let users_repository: BArc<dyn UsersRepository> = barc!(match dto {
                Some(dto) => InMemoryUserRepository::from_existing_user(dto),
                None => InMemoryUserRepository::default(),
            });
            let tmp_users_repository: BArc<dyn TemporaryUsersRepository> = barc!(match
                tmp_user_data
            {
                Some((tmp_user_id, email)) =>
                    InMemoryTmpUsersRepository::from_existing_tmp_user(tmp_user_id, email),
                None => InMemoryTmpUsersRepository::default(),
            });

            Self {
                users_repository,
                tmp_users_repository,
            }
        }
    }

    impl Default for CreateUser {
        fn default() -> Self {
            Self::new(None, None)
        }
    }

    #[tokio::test]
    async fn should_fail_to_create_user_if_no_tmp_user_exists() {
        let create_user = CreateUser::default();

        let input = CreateUserDTO {
            user_id: Uuid::new_v4(),
            first_name: "john".into(),
            last_name: "smith".into(),
            username: "johnsmith".into(),
            birthday: NaiveDate::parse_from_str("1991-12-29", "%Y-%m-%d").unwrap(),
            email: "john.smith@gmail.com".into(),
        };
        let res = create_user.execute(input).await;

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ZwitterError::not_found("TEMPORARY_USER_NOT_FOUND".into()))
    }
}
