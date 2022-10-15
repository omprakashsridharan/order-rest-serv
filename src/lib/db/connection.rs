pub use sea_orm::prelude::*;
use sea_orm::ConnectOptions;
use std::sync::Arc;

pub async fn get_connection(
    db_url: String,
) -> Result<Arc<DatabaseConnection>, Box<dyn std::error::Error>> {
    let mut connection_options = ConnectOptions::new(db_url.to_owned());
    connection_options.sqlx_logging(false);
    let connection = sea_orm::Database::connect(connection_options).await?;
    Ok(Arc::new(connection))
}
