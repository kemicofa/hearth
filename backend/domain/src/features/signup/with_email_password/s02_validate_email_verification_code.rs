use async_trait::async_trait;
use errors::ZwitterError;
use macros::BArc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dtos::signup::EmailVerificationDTO,
    features::feature::Feature,
    repositories::{
        email_verifications_repository::EmailVerificationRepository,
        tmp_users_repository::TemporaryUsersRepository,
        users_repository::UsersRepository,
    },
};

pub type ValidateEmailVerificationCodeFeature = dyn Feature<EmailVerificationDTO, Uuid>;

pub struct ValidateEmailVerificationCode {
    pub users_repository: BArc<dyn UsersRepository>,
    pub email_verifications_repository: BArc<dyn EmailVerificationRepository>,
    pub tmp_users_repository: BArc<dyn TemporaryUsersRepository>,
}

#[async_trait]
impl Feature<EmailVerificationDTO, Uuid> for ValidateEmailVerificationCode {
    async fn execute(&self, input: EmailVerificationDTO) -> Result<Uuid, errors::ZwitterError> {
        if let Err(e) = input.validate() {
            return Err(ZwitterError::Validation("VALIDATE_EMAIL_VERIFICATION_CODE".into(), e));
        }

        let is_valid = self.email_verifications_repository.code_matches(
            &input.email,
            &input.code
        ).await?;

        if !is_valid {
            return Err(ZwitterError::Domain("INVALID_EMAIL_VERIFICATION_TOKEN".into()));
        }

        let tmp_user_id = Uuid::new_v4();
        self.tmp_users_repository.store(&tmp_user_id, &input.email).await?;

        Ok(tmp_user_id)
    }
}

#[cfg(test)]
mod tests {
    use email_verification_code::EmailVerificationCode;
    use errors::ZwitterError;
    use macros::{ BArc, barc };

    use crate::{
        dtos::signup::EmailVerificationDTO,
        features::{
            feature::Feature,
            signup::with_email_password::s02_validate_email_verification_code::ValidateEmailVerificationCode,
        },
        repositories::{
            email_verifications_repository::EmailVerificationRepository,
            tmp_users_repository::TemporaryUsersRepository,
            users_repository::UsersRepository,
        },
        test_utils::test_utils::{
            InMemoryEmailVerificationRepository,
            InMemoryTmpUsersRepository,
            InMemoryUserRepository,
        },
    };

    impl Default for ValidateEmailVerificationCode {
        fn default() -> Self {
            let users_repository: BArc<dyn UsersRepository> = barc!(
                InMemoryUserRepository::default()
            );
            let email_verifications_repository: BArc<dyn EmailVerificationRepository> = barc!(
                InMemoryEmailVerificationRepository::default()
            );
            let tmp_users_repository: BArc<dyn TemporaryUsersRepository> = barc!(
                InMemoryTmpUsersRepository::default()
            );
            Self {
                users_repository,
                email_verifications_repository,
                tmp_users_repository,
            }
        }
    }

    impl ValidateEmailVerificationCode {
        fn from_email_and_code(email: String, code: EmailVerificationCode) -> Self {
            let users_repository: BArc<dyn UsersRepository> = barc!(
                InMemoryUserRepository::default()
            );
            let email_verifications_repository: BArc<dyn EmailVerificationRepository> = barc!(
                InMemoryEmailVerificationRepository::from_email_and_code(email, code)
            );
            let tmp_users_repository: BArc<dyn TemporaryUsersRepository> = barc!(
                InMemoryTmpUsersRepository::default()
            );
            Self {
                users_repository,
                email_verifications_repository,
                tmp_users_repository,
            }
        }
    }

    const EMAIL: &str = "john.smith@gmail.com";

    #[tokio::test]
    async fn should_be_able_to_validate_email_verification_code() {
        let email_verification_code = EmailVerificationCode::default();
        let s02_validate_email_verification_code =
            ValidateEmailVerificationCode::from_email_and_code(
                EMAIL.into(),
                email_verification_code.clone()
            );

        let input = EmailVerificationDTO {
            email: EMAIL.into(),
            code: email_verification_code,
        };
        let res = s02_validate_email_verification_code.execute(input).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn should_fail_if_verification_code_is_different() {
        let email_verification_code = EmailVerificationCode::from_str("ABCDEF".into()).unwrap();
        let different_email_verification_code = EmailVerificationCode::from_str(
            "123456".into()
        ).unwrap();
        let s02_validate_email_verification_code =
            ValidateEmailVerificationCode::from_email_and_code(
                EMAIL.into(),
                email_verification_code
            );

        let input = EmailVerificationDTO {
            email: EMAIL.into(),
            code: different_email_verification_code,
        };

        let res = s02_validate_email_verification_code.execute(input).await;

        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            ZwitterError::Domain("INVALID_EMAIL_VERIFICATION_TOKEN".into())
        )
    }
}
