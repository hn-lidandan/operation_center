pub use sea_orm_migration::prelude::*;
mod m20250616_094640_create_subtask_messageinfo;
mod m20250616_100023_create_main_task_messageinfo;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250616_094640_create_subtask_messageinfo::Migration),
            Box::new(m20250616_100023_create_main_task_messageinfo::Migration),
        ]
    }
}
