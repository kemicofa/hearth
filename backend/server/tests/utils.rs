use domain::dtos::signup::{EmailVerificationDTO, SignupEmailDTO};
use server::bootstrap::Dependencies;
use uuid::Uuid;

pub fn build_dependencies() -> Dependencies {
    use async_trait::async_trait;
    use domain::features::feature::Feature;
    use errors::HearthError;

    struct FakeSignupWithEmail;

    #[async_trait]
    impl Feature<SignupEmailDTO, ()> for FakeSignupWithEmail {
        async fn execute(&self, _dto: SignupEmailDTO) -> Result<(), HearthError> {
            Ok(())
        }
    }

    struct FakeSendEmailVerificationCode;

    #[async_trait]
    impl Feature<SignupEmailDTO, ()> for FakeSendEmailVerificationCode {
        async fn execute(&self, _dto: SignupEmailDTO) -> Result<(), HearthError> {
            Ok(())
        }
    }

    struct FakeValidateEmailVerificationCode;

    #[async_trait]
    impl Feature<EmailVerificationDTO, Uuid> for FakeValidateEmailVerificationCode {
        async fn execute(&self, _dto: EmailVerificationDTO) -> Result<Uuid, HearthError> {
            Ok(Uuid::new_v4())
        }
    }

    let signup_with_email = Box::new(FakeSignupWithEmail);

    Dependencies { signup_with_email }
}
