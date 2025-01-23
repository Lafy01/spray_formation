use std::error::Error;

use actix_web::web;
use chrono::{Datelike, Utc};
use num_traits::FromPrimitive;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, ActiveModelTrait, Set};

use super::{app_state::AppState, random::generate_random_id};



pub async fn new_solde (
    db: &DatabaseConnection,
    id_empl: String,
    n_matricule: String,
) -> Result<(), Box<dyn Error>> {
    let now = Utc::now();
    let current_month = now.month();
    let current_year = now.year();

    let dernier_solde = entity::solde::Entity::find()
        .filter(entity::solde::Column::IdEmpl.eq(id_empl.clone()))
        .order_by_desc(entity::solde::Column::AnneeSld)
        .order_by_desc(entity::solde::Column::MoisSld)
        .one(db)
        .await?;

    let (new_j_aqui_sld, new_j_reste_sld, j_pris) = match dernier_solde {
        Some(last_sld) => {(
            last_sld.j_aqui_sld + Decimal::from_f64(2.5).unwrap(),
            last_sld.j_reste_sld + Decimal::from_f64(2.5).unwrap(),
            last_sld.j_pris_sld
        )},
        None => {(
            Decimal::from_f64(2.5).unwrap(),
            Decimal::from_f64(2.5).unwrap(),
            Decimal::from_f64(0.0).unwrap(),
        )}
    };

    let new_solde = entity::solde::ActiveModel {
        id_sld: Set(generate_random_id(&n_matricule)),
        id_empl: Set(id_empl),
        j_aqui_sld: Set(new_j_aqui_sld),
        j_pris_sld: Set(j_pris),
        j_reste_sld: Set(new_j_reste_sld),
        mois_sld: Set(current_month as i32),
        annee_sld: Set(current_year as i32),
        ..Default::default()
    };

    new_solde.insert(db).await?;

    Ok(())
}

pub async fn soldes_employe (
    app_state: &web::Data<AppState>,
) -> Result<(), Box<dyn Error>> {
    let employes = entity::employe::Entity::find()
        .all(&app_state.db)
        .await?;

    for employe in employes {
        new_solde(&app_state.db, employe.id_empl, employe.n_matricule).await?;
    }

    Ok(())
}
