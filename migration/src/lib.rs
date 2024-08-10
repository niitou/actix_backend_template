pub use sea_orm_migration::prelude::*;

mod m20240212_023838_create_post_table;
mod m20240212_023843_create_comment_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240212_023838_create_post_table::Migration),
            Box::new(m20240212_023843_create_comment_table::Migration),
        ]
    }
}
