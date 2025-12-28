use async_trait::async_trait;
use email_verification_code::EmailVerificationCode;
use errors::ZwitterError;
use macros::BArc;
use validator::Validate;

use crate::{
    dtos::signup::SignupEmailDTO,
    features::feature::Feature,
    repositories::{
        email_sender_repository::EmailSenderRepository,
        email_verifications_repository::EmailVerificationRepository,
        users_repository::UsersRepository,
    },
};

pub type SendEmailVerificationCodeFeature = dyn Feature<SignupEmailDTO, ()>;

pub struct SendEmailVerificationCode {
    pub users_repository: BArc<dyn UsersRepository>,
    pub email_verifications_repository: BArc<dyn EmailVerificationRepository>,
    pub email_sender_repository: BArc<dyn EmailSenderRepository>,
}

#[async_trait]
impl Feature<SignupEmailDTO, ()> for SendEmailVerificationCode {
    async fn execute(&self, input: SignupEmailDTO) -> Result<(), errors::ZwitterError> {
        if let Err(e) = input.validate() {
            return Err(ZwitterError::Validation("SIGNUP_EMAIL".into(), e));
        }

        let exists = self.users_repository.exists(&input.email).await?;

        if exists {
            return Err(ZwitterError::Domain("USER_ALREADY_EXISTS".into()));
        }

        let code = EmailVerificationCode::default();

        self.email_verifications_repository.store(&input.email, &code).await?;

        self.email_sender_repository.send_verify_email(&input.email, &code).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{ NaiveDate };
    use errors::ZwitterError;
    use macros::{ BArc, barc };
    use uuid::Uuid;

    use crate::{
        dtos::{ signup::SignupEmailDTO, user::CreateUserDTO },
        features::{
            feature::Feature,
            signup::with_email_password::s01_send_email_verification_code::SendEmailVerificationCode,
        },
        repositories::{
            email_sender_repository::EmailSenderRepository,
            email_verifications_repository::EmailVerificationRepository,
            users_repository::UsersRepository,
        },
        test_utils::test_utils::{
            InMemoryEmailSenderRepository,
            InMemoryEmailVerificationRepository,
            InMemoryUserRepository,
        },
    };

    const EMAIL: &str = "john.smith@gmail.com";

    impl Default for SendEmailVerificationCode {
        fn default() -> Self {
            let users_repository: BArc<dyn UsersRepository> = barc!(
                InMemoryUserRepository::default()
            );
            let email_verifications_repository: BArc<dyn EmailVerificationRepository> = barc!(
                InMemoryEmailVerificationRepository::default()
            );
            let email_sender_repository: BArc<dyn EmailSenderRepository> = barc!(
                InMemoryEmailSenderRepository::default()
            );

            Self {
                users_repository,
                email_sender_repository,
                email_verifications_repository,
            }
        }
    }

    impl SendEmailVerificationCode {
        fn from_existing_user(dto: CreateUserDTO) -> Self {
            let users_repository: BArc<dyn UsersRepository> = barc!(
                InMemoryUserRepository::from_existing_user(dto)
            );
            let email_verifications_repository: BArc<dyn EmailVerificationRepository> = barc!(
                InMemoryEmailVerificationRepository::default()
            );
            let email_sender_repository: BArc<dyn EmailSenderRepository> = barc!(
                InMemoryEmailSenderRepository::default()
            );

            Self {
                users_repository,
                email_sender_repository,
                email_verifications_repository,
            }
        }
    }

    #[tokio::test]
    async fn should_be_able_to_complete_step_01_signup_by_email() {
        let s01_send_email_verification_code = SendEmailVerificationCode::default();

        let input = SignupEmailDTO {
            email: EMAIL.into(),
        };

        let result = s01_send_email_verification_code.execute(input).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_fail_if_a_user_with_given_email_already_exists() {
        let dto = CreateUserDTO {
            email: EMAIL.into(),
            user_id: Uuid::new_v4(),
            first_name: "John".into(),
            last_name: "Smith".into(),
            username: "johnsmith".into(),
            birthday: NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap(),
        };

        let s01_send_email_verification_code = SendEmailVerificationCode::from_existing_user(dto);

        let input = SignupEmailDTO {
            email: EMAIL.into(),
        };

        let result = s01_send_email_verification_code.execute(input).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, ZwitterError::Domain("USER_ALREADY_EXISTS".into()));
    }
}
