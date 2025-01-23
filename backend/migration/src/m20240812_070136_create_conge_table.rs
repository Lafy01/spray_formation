use sea_orm_migration::prelude::*;

use crate::m20240723_142314_create_employe_table::EMPLOYE;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CONGE::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CONGE::IdCong)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CONGE::IdEmpl).string().not_null())
                    .col(ColumnDef::new(CONGE::DateDmdCong).timestamp().not_null())
                    .col(ColumnDef::new(CONGE::MotifCong).string().not_null())
                    .col(ColumnDef::new(CONGE::DateDebCong).timestamp().not_null())
                    .col(ColumnDef::new(CONGE::DateFinCong).timestamp().not_null())
                    .col(ColumnDef::new(CONGE::NbJourCong).decimal_len(4, 1).not_null())
                    .col(ColumnDef::new(CONGE::StatusCong).string().default("En attente").not_null())
                    .col(ColumnDef::new(CONGE::DateTraitCong).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_employe")
                            .from(CONGE::Table, CONGE::IdEmpl)
                            .to(EMPLOYE::Table, EMPLOYE::IdEmpl)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CONGE::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CONGE {
    Table,
    IdCong,
    IdEmpl,
    DateDmdCong,
    MotifCong,
    DateDebCong,
    DateFinCong,
    NbJourCong,
    StatusCong,
    DateTraitCong,
}
