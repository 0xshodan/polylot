use axum::{routing::get, Router};
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = dotenv!("DATABASE_URL");
    println!("Hello, world!");
}
