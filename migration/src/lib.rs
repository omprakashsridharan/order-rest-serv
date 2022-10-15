pub use sea_orm_migration::prelude::*;

mod create_cart_table;
mod create_product_table;
mod create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_user_table::Migration),
            Box::new(create_product_table::Migration),
            Box::new(create_cart_table::Migration),
        ]
    }
}
