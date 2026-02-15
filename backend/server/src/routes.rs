use actix_web::{HttpResponse, post, web};
use domain::dtos::signup::SignupEmailDTO;
use errors::HearthError;

use crate::bootstrap::Dependencies;

#[post("/signup/email")]
pub async fn signup_email_handler(
    dependencies: web::Data<Dependencies>,
    req_body: String,
) -> Result<HttpResponse, HearthError> {
    let dto: SignupEmailDTO = serde_json::from_str(req_body.as_str()).unwrap();
    dependencies
        .signup_with_email
        .execute(dto)
        .await
        .map(|_| HttpResponse::Created().finish())
}
