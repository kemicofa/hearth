use domain::dtos::signup::{ EmailVerificationDTO, SignupEmailDTO };
use server::bootstrap::Dependencies;
use uuid::Uuid;

pub fn build_dependencies() -> Dependencies {
    use async_trait::async_trait;
    use domain::{ dtos::user::CreateUserDTO, features::feature::Feature };
    use errors::ZwitterError;

    struct FakeCreateUser;

    #[async_trait]
    impl Feature<CreateUserDTO, ()> for FakeCreateUser {
        async fn execute(&self, _dto: CreateUserDTO) -> Result<(), ZwitterError> {
            Ok(())
        }
    }

    struct FakeSendEmailVerificationCode;

    #[async_trait]
    impl Feature<SignupEmailDTO, ()> for FakeSendEmailVerificationCode {
        async fn execute(&self, _dto: SignupEmailDTO) -> Result<(), ZwitterError> {
            Ok(())
        }
    }

    struct FakeValidateEmailVerificationCode;

    #[async_trait]
    impl Feature<EmailVerificationDTO, Uuid> for FakeValidateEmailVerificationCode {
        async fn execute(&self, _dto: EmailVerificationDTO) -> Result<Uuid, ZwitterError> {
            Ok(Uuid::new_v4())
        }
    }

    Dependencies {
        create_user: Box::new(FakeCreateUser),
        send_email_verification_code: Box::new(FakeSendEmailVerificationCode),
        validate_email_verification_code: Box::new(FakeValidateEmailVerificationCode),
    }
}
