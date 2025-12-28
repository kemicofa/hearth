use std::{ sync::Arc };

use domain::{
    features::signup::with_email_password::{
        s01_send_email_verification_code::{
            SendEmailVerificationCode,
            SendEmailVerificationCodeFeature,
        },
        s02_validate_email_verification_code::{
            ValidateEmailVerificationCode,
            ValidateEmailVerificationCodeFeature,
        },
        s03_create_user::{ CreateUser, CreateUserFeature },
    },
    repositories::{
        email_sender_repository::EmailSenderRepository,
        email_verifications_repository::EmailVerificationRepository,
        tmp_users_repository::{ TemporaryUsersRepository },
        users_repository::UsersRepository,
    },
};
use macros::{ BArc, barc };
use redis::Client;
use sea_orm::DatabaseConnection;

use crate::database::{
    email_sender_repository::EmailSenderGateway,
    email_verifications_repository_redis::{ EmailVerificationsRepositoryRedis },
    tmp_users_repository_redis::TemporaryUsersRepositoryRedis,
    users_repository_postgres::UsersRepositoryPostgres,
};

pub struct DatabaseConnector {}

pub struct Dependencies {
    pub create_user: Box<CreateUserFeature>,
    pub validate_email_verification_code: Box<ValidateEmailVerificationCodeFeature>,
    pub send_email_verification_code: Box<SendEmailVerificationCodeFeature>,
}

pub fn build_dependencies(connection: DatabaseConnection, client: Client) -> Dependencies {
    // Repositories
    let users_repository: BArc<dyn UsersRepository> = barc!(
        UsersRepositoryPostgres::new(Arc::new(connection))
    );

    let email_verifications_repository: BArc<dyn EmailVerificationRepository> = barc!(
        EmailVerificationsRepositoryRedis::new(Arc::new(client))
    );

    let tmp_users_repository: BArc<dyn TemporaryUsersRepository> = barc!(
        TemporaryUsersRepositoryRedis::new()
    );

    let email_sender_repository: BArc<dyn EmailSenderRepository> = barc!(EmailSenderGateway {});

    // Features

    // Signup
    let s03_create_user = Box::new(CreateUser {
        users_repository: users_repository.clone(),
        tmp_users_repository: tmp_users_repository.clone(),
    });

    let s02_validate_email_verification_code = Box::new(ValidateEmailVerificationCode {
        users_repository: users_repository.clone(),
        email_verifications_repository: email_verifications_repository.clone(),
        tmp_users_repository: tmp_users_repository.clone(),
    });

    let s01_send_email_verification_code = Box::new(SendEmailVerificationCode {
        users_repository: users_repository.clone(),
        email_verifications_repository: email_verifications_repository.clone(),
        email_sender_repository: email_sender_repository.clone(),
    });

    Dependencies {
        create_user: s03_create_user,
        validate_email_verification_code: s02_validate_email_verification_code,
        send_email_verification_code: s01_send_email_verification_code,
    }
}
