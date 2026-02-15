use sea_orm_migration::{prelude::*, schema::*};

const TABLE_USERS: &str = "users";
const TABLE_EMAIL_PASSWORD_CREDENTIALS: &str = "email_password_credentials";
const TABLE_EMAIL_VERIFIED: &str = "email_verified";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_EMAIL_VERIFIED)
                    .col(pk_uuid("id").not_null())
                    .col(string("code").not_null())
                    .col(string("email").not_null())
                    .col(timestamp("expires_at").not_null())
                    .col(
                        timestamp("created_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TABLE_EMAIL_PASSWORD_CREDENTIALS)
                    .col(pk_uuid("id").not_null())
                    .col(string("user_id").not_null().unique_key()) // should be indexed
                    .col(string("password_hash").not_null())
                    .col(boolean("is_verified").not_null().default(false))
                    .col(
                        timestamp("created_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp("updated_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TABLE_USERS)
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(string("username").unique_key())
                    .col(string("email").unique_key())
                    .col(date("birthday").not_null())
                    .col(
                        timestamp("created_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp("updated_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TABLE_USERS).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(TABLE_EMAIL_PASSWORD_CREDENTIALS)
                    .to_owned(),
            )
            .await
    }
}
