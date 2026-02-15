use async_trait::async_trait;
use errors::HearthError;
use macros::BArc;
use validator::Validate;

use crate::{
    dtos::{auth::CredentialsDTO, signup::SignupEmailDTO, user::CreateUserDTO},
    features::feature::Feature,
    repositories::users_repository::UsersRepository,
};

pub type SignupWithEmailFeature = dyn Feature<SignupEmailDTO, ()>;

pub struct SignupWithEmail {
    pub users_repository: BArc<dyn UsersRepository>,
}

#[async_trait]
impl Feature<SignupEmailDTO, ()> for SignupWithEmail {
    async fn execute(&self, input: SignupEmailDTO) -> Result<(), errors::HearthError> {
        if let Err(e) = input.validate() {
            return Err(HearthError::Validation("SIGNUP_EMAIL".into(), e));
        }

        {
            let exists = self.users_repository.email_exists(&input.email).await?;

            if exists {
                return Err(HearthError::Domain("EMAIL_ALREADY_TAKEN".into()));
            }
        }

        {
            let exists = self
                .users_repository
                .username_exists(&input.username)
                .await?;

            if exists {
                return Err(HearthError::Domain("USERNAME_ALREADY_TAKEN".into()));
            }
        }

        let create_user_dto = CreateUserDTO {
            user_id: input.user_id,
            username: input.username.clone(),
            birthday: input.birthday,
            email: input.email,
        };

        let credentials_dto = CredentialsDTO {
            user_id: input.user_id,
            password: input.password,
        };

        self.users_repository
            .create(create_user_dto, credentials_dto)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dtos::auth::CredentialsDTO,
        features::{feature::Feature, signup::signup_with_email::SignupWithEmail},
    };
    use chrono::NaiveDate;
    use errors::HearthError;
    use macros::{BArc, barc};
    use uuid::Uuid;

    use crate::{
        dtos::{signup::SignupEmailDTO, user::CreateUserDTO},
        repositories::users_repository::UsersRepository,
        test_utils::test_utils::InMemoryUserRepository,
    };

    const EMAIL: &str = "john.smith@gmail.com";
    const USERNAME: &str = "john.smith";
    const PASSWORD: &str = "qwerty123";

    impl Default for SignupEmailDTO {
        fn default() -> Self {
            Self {
                user_id: Uuid::new_v4(),
                email: EMAIL.into(),
                username: USERNAME.into(),
                password: "qwerty123".into(),
                birthday: NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap(),
            }
        }
    }

    impl Default for SignupWithEmail {
        fn default() -> Self {
            let users_repository: BArc<dyn UsersRepository> =
                barc!(InMemoryUserRepository::default());
            Self { users_repository }
        }
    }

    impl SignupWithEmail {
        fn from_existing_user(dto: CreateUserDTO, credentials_dto: CredentialsDTO) -> Self {
            let users_repository: BArc<dyn UsersRepository> = barc!(
                InMemoryUserRepository::from_existing_user(dto, credentials_dto)
            );

            Self { users_repository }
        }
    }

    #[tokio::test]
    async fn should_be_able_to_signup_with_email() {
        let s01_send_email_verification_code = SignupWithEmail::default();

        let input = SignupEmailDTO::default();

        let result = s01_send_email_verification_code.execute(input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_fail_if_a_user_with_given_email_already_exists() {
        let input = SignupEmailDTO::default();
        let user_id = Uuid::new_v4();
        let dto = CreateUserDTO {
            user_id,
            username: input.username.clone(),
            birthday: input.birthday.clone(),
            email: input.email.clone(),
        };

        let credentials_dto = CredentialsDTO {
            user_id,
            password: PASSWORD.into(),
        };

        let signup_with_email = SignupWithEmail::from_existing_user(dto, credentials_dto);

        let result = signup_with_email.execute(input).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, HearthError::Domain("EMAIL_ALREADY_TAKEN".into()));
    }

    #[tokio::test]
    async fn should_fail_if_a_user_with_given_username_already_exists() {
        let user_id = Uuid::new_v4();

        let input = SignupEmailDTO::default();

        let dto = CreateUserDTO {
            user_id,
            username: input.username.clone(),
            birthday: input.birthday.clone(),
            email: "john.smith2@gmail.com".into(),
        };

        let credentials_dto = CredentialsDTO {
            user_id,
            password: PASSWORD.into(),
        };

        let signup_with_email = SignupWithEmail::from_existing_user(dto, credentials_dto);

        let input = SignupEmailDTO::default();

        let result = signup_with_email.execute(input).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, HearthError::Domain("USERNAME_ALREADY_TAKEN".into()));
    }
}
