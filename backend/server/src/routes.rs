use actix_web::{ HttpResponse, http::Error, post, web };
use domain::{ dtos::user::CreateUserDTO };
use errors::ZwitterError;

use crate::bootstrap::Dependencies;

#[post("/users")]
pub async fn create_user_handler(
    dependencies: web::Data<Dependencies>,
    req_body: String
) -> Result<HttpResponse, ZwitterError> {
    let dto: CreateUserDTO = serde_json::from_str(req_body.as_str()).unwrap();
    dependencies.create_user.execute(dto).await.map(|_| HttpResponse::Created().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{ App, body::MessageBody, http::StatusCode, test, web };

    use crate::{ bootstrap::build_dependencies, routes::create_user_handler };

    #[actix_web::test]
    async fn should_be_able_call_create_user() {
        let dependencies = web::Data::new(build_dependencies());
        let app = test::init_service(
            App::new().app_data(dependencies).service(create_user_handler)
        ).await;

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
        let req = test::TestRequest::post().uri("/users").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
