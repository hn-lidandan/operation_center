use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SubtaskMessageinfo::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SubtaskMessageinfo::Id).big_integer().not_null().primary_key())
                    .col(ColumnDef::new(SubtaskMessageinfo::SubTitle).string().string_len(255).not_null())
                    .col(ColumnDef::new(SubtaskMessageinfo::Description).text())
                    .col(ColumnDef::new(SubtaskMessageinfo::Status).string().string_len(50).not_null())
                    .col(ColumnDef::new(SubtaskMessageinfo::Log).text())
                    .col(ColumnDef::new(SubtaskMessageinfo::TaskOrder).integer())
                    .col(ColumnDef::new(SubtaskMessageinfo::TaskType).string().string_len(200).not_null())
                    .col(ColumnDef::new(SubtaskMessageinfo::ParentId).big_integer())
                    .col(ColumnDef::new(SubtaskMessageinfo::CreateTimestamp).timestamp_with_time_zone())
                    .col(ColumnDef::new(SubtaskMessageinfo::UpdateTimestamp).timestamp_with_time_zone())
                    .col(ColumnDef::new(SubtaskMessageinfo::Bak1).big_integer())
                    .col(ColumnDef::new(SubtaskMessageinfo::Bak2).string().string_len(255))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SubtaskMessageinfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SubtaskMessageinfo {
    Table,
    Id,
    SubTitle,
    Description,
    Status,
    Log,
    TaskOrder,
    TaskType,
    ParentId,
    CreateTimestamp,
    UpdateTimestamp,
    Bak1,
    Bak2,
}
