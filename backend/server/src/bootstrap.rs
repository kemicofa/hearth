use std::sync::Arc;

use domain::{
    features::signup::signup_with_email::{SignupWithEmail, SignupWithEmailFeature},
    repositories::{
        email_sender_repository::EmailSenderRepository,
        email_verifications_repository::EmailVerificationRepository,
        users_repository::UsersRepository,
    },
};
use macros::{BArc, barc};
use redis::Client;
use sea_orm::DatabaseConnection;

use crate::database::{
    email_sender_repository::EmailSenderGateway,
    email_verifications_repository_redis::EmailVerificationsRepositoryRedis,
    users_repository_postgres::UsersRepositoryPostgres,
};

pub struct DatabaseConnector {}

pub struct Dependencies {
    pub signup_with_email: Box<SignupWithEmailFeature>,
}

pub fn build_dependencies(connection: DatabaseConnection, client: Client) -> Dependencies {
    // Repositories
    let users_repository: BArc<dyn UsersRepository> =
        barc!(UsersRepositoryPostgres::new(Arc::new(connection)));

    // let email_verifications_repository: BArc<dyn EmailVerificationRepository> = barc!(
    //     EmailVerificationsRepositoryRedis::new(Arc::new(client))
    // );

    // let email_sender_repository: BArc<dyn EmailSenderRepository> = barc!(EmailSenderGateway {});

    // Features

    // Signup
    let signup_with_email = Box::new(SignupWithEmail {
        users_repository: users_repository.clone(),
    });

    Dependencies { signup_with_email }
}
