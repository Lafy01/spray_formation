pub use sea_orm_migration::prelude::*;

mod m20240723_134332_create_departement_table;
mod m20240723_142314_create_employe_table;
mod m20240808_074507_create_foreign_keys;
mod m20240812_055100_create_solde_table;
mod m20240812_070136_create_conge_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240723_134332_create_departement_table::Migration),
            Box::new(m20240723_142314_create_employe_table::Migration),
            Box::new(m20240808_074507_create_foreign_keys::Migration),
            Box::new(m20240812_055100_create_solde_table::Migration),
            Box::new(m20240812_070136_create_conge_table::Migration),
        ]
    }
}
