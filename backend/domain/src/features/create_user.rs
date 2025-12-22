use async_trait::async_trait;
use errors::ZwitterError;
use macros::BArc;
use validator::Validate;

use crate::{
    dtos::user::CreateUserDTO,
    features::feature::Feature,
    repositories::users_repository::UsersRepository,
};

pub struct CreateUser {
    pub users_repository: BArc<dyn UsersRepository>,
}

pub type CreateUserFeature = dyn Feature<CreateUserDTO, ()>;

#[async_trait]
impl Feature<CreateUserDTO, ()> for CreateUser {
    async fn execute(&self, input: CreateUserDTO) -> Result<(), errors::ZwitterError> {
        if let Err(e) = input.validate() {
            return Err(ZwitterError::Validation("CREATE_USER".into(), e));
        }

        self.users_repository.create(input).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use errors::ZwitterError;
    use macros::{ BArc, barc };

    use crate::{
        dtos::user::CreateUserDTO,
        features::{ create_user::CreateUser, feature::Feature },
        repositories::users_repository::UsersRepository,
        test_utils::test_utils::InMemoryUserRepository,
    };

    #[tokio::test]
    async fn should_return_validation_errors() {
        let create_user = CreateUser::default();

        let payload =
            r#"
            {
                "user_id": "47578122-3977-438a-8e2c-1f1f4fe8b7ef",
                "first_name": "",
                "last_name": "SmithSmithSmithSmithSmithSmithSmithSmithSmithSmithSmithSmithSmithSmith",
                "username": "jo",
                "birthday": "1993-12-29",
                "email": "john.smith"
            } 
        "#;

        let create_user_dto: CreateUserDTO = serde_json::from_str(payload).unwrap();

        let result = create_user.execute(create_user_dto).await;

        assert!(result.is_err());

        let error = result.unwrap_err();

        let expected_error: ZwitterError = serde_json
            ::from_str(
                "{\"Validation\":[\"CREATE_USER\",{\"email\":[{\"code\":\"email\",\"message\":null,\"params\":{\"value\":\"john.smith\"}}],\"first_name\":[{\"code\":\"length\",\"message\":null,\"params\":{\"min\":1,\"max\":50,\"value\":\"\"}}],\"username\":[{\"code\":\"length\",\"message\":null,\"params\":{\"value\":\"jo\",\"min\":3,\"max\":24}}]}]}"
            )
            .unwrap();
        assert_eq!(error, expected_error)
    }

    #[tokio::test]
    async fn should_be_able_create_a_user() {
        let users_repository: BArc<dyn UsersRepository> = barc!(InMemoryUserRepository::default());
        let create_user = CreateUser {
            users_repository: users_repository.clone(),
        };

        let payload =
            r#"
            {
                "user_id": "47578122-3977-438a-8e2c-1f1f4fe8b7ef",
                "first_name": "John",
                "last_name": "Smith",
                "username": "johnsmith",
                "birthday": "1991-12-29",
                "email": "john.smith@gmail.com"
            } 
        "#;

        let create_user_dto: CreateUserDTO = serde_json::from_str(payload).unwrap();

        let result = create_user.execute(create_user_dto.clone()).await;

        assert!(result.is_ok());

        let user_dto = users_repository.get(create_user_dto.user_id.into()).await.unwrap();

        assert_eq!(user_dto.user_id, create_user_dto.user_id);
        assert_eq!(user_dto.birthday.to_string(), "1991-12-29");
    }
}
