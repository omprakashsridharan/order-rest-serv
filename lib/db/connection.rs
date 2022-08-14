pub use sea_orm::prelude::*;
use sea_orm::ConnectOptions;
use std::env;

pub async fn get_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection_options = ConnectOptions::new(db_url.to_owned());
    connection_options.sqlx_logging(false);
    let connection = sea_orm::Database::connect(connection_options).await?;
    Ok(connection)
}
