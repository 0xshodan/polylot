use axum::{Router, Server};
use migration::{Migrator, MigratorTrait};
use repository::sea_orm::Database;
use std::{env, net::SocketAddr, str::FromStr};

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    let debug = env::var("DEBUG").unwrap_or("false".to_string());
    if debug == "true" {
        env::set_var("RUST_LOG", "debug");
    }

    // tracing_subscriber::fmt::init();

    // dotenvy::dotenv().ok();
    let db_url = {
        let db_type = env::var("DATABASE_TYPE").unwrap_or("postgres".to_string());
        let db_host = env::var("DATABASE_HOST").unwrap_or("db".to_string());
        let db_port = env::var("DATABASE_PORT").unwrap_or("5432".to_string());
        let db_user = env::var("DATABASE_USER").expect("DATABASE_USER is not set in .env file");
        let db_password =
            env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD is not set in .env file");
        let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
        format!("{db_type}://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}")
    };
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let server_url = format!("{host}:{port}");
    let addr = SocketAddr::from_str(&server_url).unwrap();
    let app = Router::new();
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
