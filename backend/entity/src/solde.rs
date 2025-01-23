//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "solde")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_sld: String,
    pub id_empl: String,
    #[sea_orm(column_type = "Decimal(Some((4, 1)))")]
    pub j_aqui_sld: Decimal,
    #[sea_orm(column_type = "Decimal(Some((4, 1)))")]
    pub j_pris_sld: Decimal,
    #[sea_orm(column_type = "Decimal(Some((4, 1)))")]
    pub j_reste_sld: Decimal,
    pub mois_sld: i32,
    pub annee_sld: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::employe::Entity",
        from = "Column::IdEmpl",
        to = "super::employe::Column::IdEmpl",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Employe,
}

impl Related<super::employe::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employe.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
