mod entity;
mod migration;
mod repository;
mod utils;
use crate::migration::{Migrator, MigratorTrait};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DB url {}", db_url.clone());
    let connection = sea_orm::Database::connect(&db_url).await?;
    Migrator::up(&connection, None).await?;
    Ok(())
}
