use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MainTaskMessageinfo::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MainTaskMessageinfo::Id).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(MainTaskMessageinfo::Name).string().string_len(255).not_null())
                    .col(ColumnDef::new(MainTaskMessageinfo::Description).text())
                    .col(ColumnDef::new(MainTaskMessageinfo::Status).string().string_len(50).not_null())
                    .col(ColumnDef::new(MainTaskMessageinfo::Log).text())
                    .col(ColumnDef::new(MainTaskMessageinfo::Priority).integer())
                    .col(ColumnDef::new(MainTaskMessageinfo::TaskType).string().string_len(200).not_null())
                    .col(ColumnDef::new(MainTaskMessageinfo::WorkerName).string().string_len(255).not_null())
                    .col(ColumnDef::new(MainTaskMessageinfo::CreateTimestamp).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(MainTaskMessageinfo::UpdateTimestamp).timestamp_with_time_zone())
                    .col(ColumnDef::new(MainTaskMessageinfo::Bak1).big_integer())
                    .col(ColumnDef::new(MainTaskMessageinfo::Bak2).string().string_len(255))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(MainTaskMessageinfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MainTaskMessageinfo {
    Table,
    Id,
    Name,
    Description,
    Status,
    Log,
    Priority,
    TaskType,
    WorkerName,
    CreateTimestamp,
    UpdateTimestamp,
    Bak1,
    Bak2,
}
