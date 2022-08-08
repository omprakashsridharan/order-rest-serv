pub use sea_orm::prelude::*;
use std::env;

pub async fn get_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = sea_orm::Database::connect(db_url).await?;
    Ok(connection)
}
