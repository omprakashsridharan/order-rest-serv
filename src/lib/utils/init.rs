use crate::db::connection::get_connection;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;
use tracing::info;

pub async fn initialise<M: MigratorTrait>(
    db_url: String,
) -> Result<Arc<DatabaseConnection>, Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let connection = get_connection(db_url).await?.clone();
    info!("Establishing DB connection");
    <M as MigratorTrait>::up(&connection, None).await?;
    info!("DB Migration complete");
    Ok(connection)
}
