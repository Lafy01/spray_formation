use sea_orm_migration::prelude::*;

use crate::{m20240723_134332_create_departement_table::DEPARTEMENT, m20240723_142314_create_employe_table::EMPLOYE};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_departement")
                    .from(EMPLOYE::Table, EMPLOYE::IdDep)
                    .to(DEPARTEMENT::Table, DEPARTEMENT::IdDep)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
            
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_suphier")
                    .from(DEPARTEMENT::Table, DEPARTEMENT::ChefDep)
                    .to(EMPLOYE::Table, EMPLOYE::IdEmpl)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_departement")
                    .table(EMPLOYE::Table)
                    .to_owned()
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_suphier")
                    .table(DEPARTEMENT::Table)
                    .to_owned()
            )
            .await?;
        Ok(())
    }
}