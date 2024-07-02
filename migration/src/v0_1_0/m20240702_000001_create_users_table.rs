use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Name)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(320)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Password).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,

    Id,
    Name,
    Email,
    Password,
}
