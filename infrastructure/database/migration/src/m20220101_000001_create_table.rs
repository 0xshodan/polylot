use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql_list = if manager.get_database_backend() == DbBackend::Postgres {
            let users = r#"
                CREATE TABLE IF NOT EXISTS "users" (
                    "id" serial PRIMARY KEY,
                    "username" varchar(32) UNIQUE,
                    "email" varchar(320) NOT NULL UNIQUE,
                    "password" char(60) NOT NULL,
                    "created_at" timestamp NOT NULL DEFAULT NOW()
                );"#;
            let modules = r#"
                CREATE TABLE IF NOT EXISTS "modules" (
                    "id" serial PRIMARY KEY,
                    "user_id" integer NOT NULL,
                    "name" text NOT NULL,
                    "description" text,
                    "created_at" timestamp NOT NULL DEFAULT NOW(),
                    FOREIGN KEY (user_id) REFERENCES users (id)
                );"#;
            let cards = r#"
                CREATE TABLE IF NOT EXISTS "cards" (
                    "id" serial PRIMARY KEY,
                    "module_id" integer NOT NULL,
                    "name" text NOT NULL,
                    FOREIGN KEY (module_id) REFERENCES modules (id)
                );"#;
            let definitions = r#"
                CREATE TABLE IF NOT EXISTS "definitions" (
                    "id" serial PRIMARY KEY,
                    "card_id" integer NOT NULL,
                    "picture" text,
                    "meaning" text,
                    "meaning_native" text,
                    "use_cases" text ARRAY,
                    FOREIGN KEY (card_id) REFERENCES cards (id)
                );"#;
            Some([users, modules, cards, definitions])
        } else {
            None
        };

        if let Some(sql_list) = sql_list {
            for sql in sql_list {
                manager
                    .get_connection()
                    .execute(Statement::from_string(
                        manager.get_database_backend(),
                        sql.to_owned().to_string(),
                    ))
                    .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
