use actix_web::{App, http::StatusCode, test, web};
use server::routes::signup_email_handler;

use crate::utils::build_dependencies;

#[actix_web::test]
async fn should_be_able_to_signup_with_email() {
    let dependencies = web::Data::new(build_dependencies());
    let app = test::init_service(
        App::new()
            .app_data(dependencies)
            .service(signup_email_handler),
    )
    .await;

    let payload = r#"
                {
                    "user_id": "47578122-3977-438a-8e2c-1f1f4fe8b7ef",
                    "first_name": "John",
                    "last_name": "Smith",
                    "username": "johnsmith",
                    "birthday": "1991-12-29",
                    "email": "john.smith@gmail.com",
                    "password": "qwerty123"
                }
            "#;
    let req = test::TestRequest::post()
        .uri("/signup/email")
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
}
