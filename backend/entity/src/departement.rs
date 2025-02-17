//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "departement")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_dep: String,
    #[sea_orm(unique)]
    pub code_dep: String,
    #[sea_orm(unique)]
    pub nom_dep: String,
    pub date_deb_form: Option<DateTime>,
    pub date_fin_form: Option<DateTime>,
    #[sea_orm(unique)]
    pub chef_dep: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::employe::Entity",
        from = "Column::ChefDep",
        to = "super::employe::Column::IdEmpl",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Employe,
}

impl Related<super::employe::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employe.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
