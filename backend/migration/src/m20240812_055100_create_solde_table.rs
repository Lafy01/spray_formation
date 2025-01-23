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
                    .table(SOLDE::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SOLDE::IdSld)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SOLDE::IdEmpl).string().not_null())
                    .col(
                        ColumnDef::new(SOLDE::JAquiSld)
                            .decimal_len(4, 1)
                            .not_null()
                            .default(50.5),
                    )
                    .col(
                        ColumnDef::new(SOLDE::JPrisSld)
                            .decimal_len(4, 1)
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(SOLDE::JResteSld)
                            .decimal_len(4, 1)
                            .not_null()
                            .default(50.5),
                    )
                    .col(ColumnDef::new(SOLDE::MoisSld).integer().not_null())
                    .col(ColumnDef::new(SOLDE::AnneeSld).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_employe")
                            .from(SOLDE::Table, SOLDE::IdEmpl)
                            .to(EMPLOYE::Table, EMPLOYE::IdEmpl)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SOLDE::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SOLDE {
    Table,
    IdSld,
    IdEmpl,
    JAquiSld,
    JPrisSld,
    JResteSld,
    MoisSld,
    AnneeSld,
}
