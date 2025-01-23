use sea_orm_migration::prelude::*;

// use crate::m20240723_142314_create_employe_table::EMPLOYE;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(DEPARTEMENT::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DEPARTEMENT::IdDep)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DEPARTEMENT::CodeDep).string().not_null().unique_key())
                    .col(ColumnDef::new(DEPARTEMENT::NomDep).string().not_null().unique_key())
                    .col(ColumnDef::new(DEPARTEMENT::DateDebForm).timestamp())
                    .col(ColumnDef::new(DEPARTEMENT::DateFinForm).timestamp())
                    .col(ColumnDef::new(DEPARTEMENT::ChefDep).string().unique_key())
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .name("fk_suphier")
                    //         .from(DEPARTEMENT::Table, DEPARTEMENT::SupHier)
                    //         .to(EMPLOYE::Table, EMPLOYE::IdEmpl)
                    //         .on_delete(ForeignKeyAction::SetNull)
                    //         .on_update(ForeignKeyAction::SetNull)
                    // )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DEPARTEMENT::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DEPARTEMENT {
    Table,
    IdDep,
    CodeDep,
    NomDep,
    DateDebForm,
    DateFinForm,
    ChefDep,
}
