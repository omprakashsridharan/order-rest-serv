use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;
use tracing::info;

pub async fn initialise<M: MigratorTrait>(
    db_url: String,
) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!("DB url {}", db_url);
    let connection = sea_orm::Database::connect(db_url).await?;
    <M as MigratorTrait>::up(&connection, None).await?;
    Ok(connection)
}
