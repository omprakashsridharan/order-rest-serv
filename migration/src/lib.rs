pub use sea_orm_migration::prelude::*;

mod create_cart_table;
mod create_product_table;
mod create_user_table;

pub struct AuthMigrator;

#[async_trait::async_trait]
impl MigratorTrait for AuthMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(create_user_table::Migration)]
    }
}

pub struct InventoryhMigrator;

#[async_trait::async_trait]
impl MigratorTrait for InventoryhMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(create_product_table::Migration)]
    }
}

pub struct OrderhMigrator;

#[async_trait::async_trait]
impl MigratorTrait for OrderhMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(create_cart_table::Migration)]
    }
}
