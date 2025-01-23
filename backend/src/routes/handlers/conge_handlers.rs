use std::str::FromStr;

use actix_web::{get, post, put, web};
use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone, Utc};
use num_traits::cast::ToPrimitive;
use sea_orm::{prelude::Decimal, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};

use crate::{
    routes::handlers::employe_handlers::UserInfo,
    utils::{api_response::ApiResponse, app_state, date_fin::calculer_date_fin_conge, jwt::Claims, random::generate_random_id},
};

use super::employe_handlers::DepartementInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct CongeWithEmploye {
    pub id_cong: String,
    pub date_dmd_cong: chrono::DateTime<chrono::Local>,
    pub motif_cong: String,
    pub date_deb_cong: chrono::NaiveDateTime,
    pub date_fin_cong: chrono::NaiveDateTime,
    pub nb_jour_cong: Decimal,
    pub status_cong: String,
    pub date_trait_cong: Option<chrono::DateTime<chrono::Local>>,
    pub employe: Option<UserInfo>,
    pub departement: Option<DepartementInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CongePlanning {
    pub id_cong: String,
    pub date_dmd_cong: chrono::NaiveDateTime,
    pub motif_cong: String,
    pub date_deb_cong: chrono::NaiveDateTime,
    pub date_fin_cong: chrono::NaiveDateTime,
    pub nb_jour_cong: Decimal,
    pub status_cong: String,
    pub date_trait_cong: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CongeData {
    pub id_empl: String,
    pub motif_cong: String,
    pub date_deb_cong: DateTime<Utc>,
    pub debut_matin: bool,
    pub nb_jour_cong: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyCongeData {
    pub motif_cong: String,
    pub date_deb_cong: DateTime<Utc>,
    pub debut_matin: bool,
    pub nb_jour_cong: Decimal,
}

#[post("/add_my_conge")]
pub async fn add_my_conge(
    app_state: web::Data<app_state::AppState>,
    data: web::Json<MyCongeData>,
    claim_data: Claims,
) -> Result<ApiResponse<CongeData>, ApiResponse<()>> {
    let date_deb_cong_utc = data.date_deb_cong.naive_local();
    let date_fin_cong = calculer_date_fin_conge(date_deb_cong_utc, data.nb_jour_cong.to_f64().unwrap());

    let now = Local::now();

    let id_conge = generate_random_id(&claim_data.id);

    let solde_result = entity::solde::Entity::find()
        .filter(entity::solde::Column::IdEmpl.eq(claim_data.id.clone()))
        .order_by_desc(entity::solde::Column::AnneeSld)
        .order_by_desc(entity::solde::Column::MoisSld)
        .one(&app_state.db)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            ApiResponse::new(500, false, err.to_string(), None)
        })?;
    
    let mut solde = match solde_result {
        Some(solde) => solde.into_active_model(),
        None => {
            return Err(ApiResponse::new(
                200,
                false,
                "Solde not found".to_string(),
                None,
            ));
        }
    };

    let nb_jours_f64 = data.nb_jour_cong.to_f64().unwrap().to_string();
    let nb_jours_decimal = Decimal::from_str(&nb_jours_f64)
        .map_err(|_| {
            ApiResponse::new(500, false, "Failed to parse nb_jours_cong".to_string(), None)
        })?;
    let nouveau_j_pris_sld = solde.j_pris_sld.as_ref() + nb_jours_decimal;
    let nouveau_j_restant_sld = solde.j_reste_sld.as_ref() - nb_jours_decimal;

    if nouveau_j_restant_sld < Decimal::from_str("0.0").unwrap() {
        return Err(ApiResponse::new(
            200,
            false,
            "Solde insuffisant".to_string(),
            None,
        ));
    }
    solde.j_pris_sld = Set(nouveau_j_pris_sld);
    solde.j_reste_sld = Set(nouveau_j_restant_sld);

    solde
        .update(&app_state.db)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            ApiResponse::new(500, false, err.to_string(), None)
        })?;

    let departement = entity::departement::Entity::find()
        .filter(entity::departement::Column::ChefDep.eq(claim_data.id.clone()))
        .one(&app_state.db)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            ApiResponse::new(500, false, err.to_string(), None)
        })?;
    
    let status_conge = match departement {
        Some(_dept) => "En attente RH".to_string(),
        None => "En attente chef DTP".to_string(),
    };

    let new_conge = entity::conge::ActiveModel {
        id_cong: Set(id_conge),
        id_empl: Set(claim_data.id.clone()),
        date_dmd_cong: Set(now.naive_utc()),
        motif_cong: Set(data.motif_cong.to_owned()),
        date_deb_cong: Set(date_deb_cong_utc),
        date_fin_cong: Set(date_fin_cong),
        nb_jour_cong: Set(data.nb_jour_cong),
        status_cong: Set(status_conge),
        date_trait_cong: Set(None),
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| {
        println!("{:?}", err);
        ApiResponse::new(500, false, err.to_string(), None)
    })?;

    let conge_data = CongeData {
        id_empl: claim_data.id,
        motif_cong: new_conge.motif_cong,
        date_deb_cong: Utc.from_utc_datetime(&new_conge.date_deb_cong),
        debut_matin: data.debut_matin,
        nb_jour_cong: new_conge.nb_jour_cong,
    };

    Ok(ApiResponse::new(
        200,
        true,
        "Conge added successfully".to_string(),
        Some(conge_data),
    ))
}

#[post("/add_conge")]
pub async fn add_conge(
    app_state: web::Data<app_state::AppState>,
    data: web::Json<CongeData>,
) -> Result<ApiResponse<CongeData>, ApiResponse<()>> {
    let date_deb_cong_utc = data.date_deb_cong;
    let nb_jours = data.nb_jour_cong.to_f64().unwrap();
    let days = nb_jours.trunc() as i64;
    let half_day = (nb_jours.fract() * 2.0).round() as i64;

    let mut date_fin_cong = date_deb_cong_utc + Duration::days(days - 1);

    if data.debut_matin {
        date_fin_cong =
            date_fin_cong + Duration::hours(((12.0 * (half_day as f64 + 1.0)).round()) as i64);
    } else {
        date_fin_cong = date_fin_cong
            + Duration::hours(((12.0 * half_day as f64).round()) as i64)
            + Duration::hours(18);
    }

    let now = Local::now();

    let id_conge = generate_random_id(&data.id_empl);

    let new_conge = entity::conge::ActiveModel {
        id_cong: Set(id_conge),
        id_empl: Set(data.id_empl.to_owned()),
        date_dmd_cong: Set(now.naive_utc()),
        motif_cong: Set(data.motif_cong.to_owned()),
        date_deb_cong: Set(date_deb_cong_utc.naive_utc()),
        date_fin_cong: Set(date_fin_cong.naive_utc()),
        nb_jour_cong: Set(data.nb_jour_cong),
        status_cong: Set("En attente".to_string()),
        date_trait_cong: Set(None),
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| {
        println!("{:?}", err);
        ApiResponse::new(500, false, err.to_string(), None)
    })?;

    let conge_data = CongeData {
        id_empl: new_conge.id_empl,
        motif_cong: new_conge.motif_cong,
        date_deb_cong: Utc.from_utc_datetime(&new_conge.date_deb_cong),
        debut_matin: data.debut_matin,
        nb_jour_cong: new_conge.nb_jour_cong,
    };

    Ok(ApiResponse::new(
        200,
        true,
        "Conge added successfully".to_string(),
        Some(conge_data),
    ))
}


#[get("/all_my_conge")]
pub async fn all_my_conge(
    app_state: web::Data<app_state::AppState>,
    claims: Claims,
) -> Result<ApiResponse<Vec<CongeWithEmploye>>, ApiResponse<()>> {
    let conges = entity::conge::Entity::find()
        .find_also_related(entity::employe::Entity)
        .filter(entity::conge::Column::IdEmpl.eq(claims.id))
        .all(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;
        
    let conge_models = futures::future::join_all(conges
        .into_iter()
        .map(|(conge, employe)| {
            let app_state = app_state.clone();
            async move {
                let departement = if let Some(empl) = employe.as_ref() {
                    entity::departement::Entity::find_by_id(empl.id_dep.clone())
                        .one(&app_state.db)
                        .await
                        .map_err(|e| ApiResponse::new(
                            500,
                            false,
                            e.to_string(),
                            None
                        ))?
                } else {
                    None
                };

                Ok(CongeWithEmploye {
                    id_cong: conge.id_cong,
                    date_dmd_cong: Local.from_local_datetime(&conge.date_dmd_cong).unwrap(),
                    motif_cong: conge.motif_cong,
                    date_deb_cong: conge.date_deb_cong,
                    date_fin_cong: conge.date_fin_cong,
                    nb_jour_cong: conge.nb_jour_cong,
                    status_cong: conge.status_cong,
                    date_trait_cong: conge.date_trait_cong.map(|dt| Local.from_local_datetime(&dt).unwrap()),
                    employe: employe.map(|empl| UserInfo {
                        n_matricule: empl.n_matricule,
                        id_dep: empl.id_dep,
                        nom_empl: empl.nom_empl,
                        prenom_empl: empl.prenom_empl,
                        email_empl: empl.email_empl,
                        role: empl.role,
                    }),
                    departement: departement.map(|dept| DepartementInfo {
                        code_dep: dept.code_dep,
                        nom_dep: dept.nom_dep,
                    })
                })
            }
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ApiResponse::new(
        200,
        true,
        "All conges with employee Information".to_string(),
        Some(conge_models),
    ))
}



#[get("/all_my_employe_conge")]
pub async fn all_my_employe_conge(
    app_state: web::Data<app_state::AppState>,
    claims: Claims,
) -> Result<ApiResponse<Vec<CongeWithEmploye>>, ApiResponse<()>> {

    let chef_departement = entity::departement::Entity::find()
        .filter(entity::departement::Column::ChefDep.eq(claims.id.clone()))
        .one(&app_state.db)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            ApiResponse::new(500, false, "Server Error".to_string(), None)
        })?;

    let chef_departement_id = match chef_departement {
        Some(dept) => dept.id_dep,
        None => {
            return Ok(ApiResponse::new(
                200,
                false,
                "Chef de departement not found".to_string(),
                None
            ));
        }
    };    

    let conges = entity::conge::Entity::find()
        .find_also_related(entity::employe::Entity)
        .filter(entity::conge::Column::IdEmpl.ne(claims.id))
        .filter(entity::employe::Column::IdDep.eq(chef_departement_id))
        .all(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;
        
    let conge_models = futures::future::join_all(conges
        .into_iter()
        .map(|(conge, employe)| {
            let app_state = app_state.clone();
            async move {
                let departement = if let Some(empl) = employe.as_ref() {
                    entity::departement::Entity::find_by_id(empl.id_dep.clone())
                        .one(&app_state.db)
                        .await
                        .map_err(|e| ApiResponse::new(
                            500,
                            false,
                            e.to_string(),
                            None
                        ))?
                } else {
                    None
                };

                Ok(CongeWithEmploye {
                    id_cong: conge.id_cong,
                    date_dmd_cong: Local.from_local_datetime(&conge.date_dmd_cong).unwrap(),
                    motif_cong: conge.motif_cong,
                    date_deb_cong: conge.date_deb_cong,
                    date_fin_cong: conge.date_fin_cong,
                    nb_jour_cong: conge.nb_jour_cong,
                    status_cong: conge.status_cong,
                    date_trait_cong: conge.date_trait_cong.map(|dt| Local.from_local_datetime(&dt).unwrap()),
                    employe: employe.map(|empl| UserInfo {
                        n_matricule: empl.n_matricule,
                        id_dep: empl.id_dep,
                        nom_empl: empl.nom_empl,
                        prenom_empl: empl.prenom_empl,
                        email_empl: empl.email_empl,
                        role: empl.role,
                    }),
                    departement: departement.map(|dept| DepartementInfo {
                        code_dep: dept.code_dep,
                        nom_dep: dept.nom_dep,
                    })
                })
            }
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ApiResponse::new(
        200,
        true,
        "All conges with employee Information".to_string(),
        Some(conge_models),
    ))
}

#[get("/all_conge")]
pub async fn all_conge(
    app_state: web::Data<app_state::AppState>,
) -> Result<ApiResponse<Vec<CongeWithEmploye>>, ApiResponse<()>> {
    let conges = entity::conge::Entity::find()
        .find_also_related(entity::employe::Entity)
        .all(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;
        
    let conge_models = futures::future::join_all(conges
        .into_iter()
        .map(|(conge, employe)| {
            let app_state = app_state.clone();
            async move {
                let departement = if let Some(empl) = employe.as_ref() {
                    entity::departement::Entity::find_by_id(empl.id_dep.clone())
                        .one(&app_state.db)
                        .await
                        .map_err(|e| ApiResponse::new(
                            500,
                            false,
                            e.to_string(),
                            None
                        ))?
                } else {
                    None
                };

                Ok(CongeWithEmploye {
                    id_cong: conge.id_cong,
                    date_dmd_cong: Local.from_local_datetime(&conge.date_dmd_cong).unwrap(),
                    motif_cong: conge.motif_cong,
                    date_deb_cong: conge.date_deb_cong,
                    date_fin_cong: conge.date_fin_cong,
                    nb_jour_cong: conge.nb_jour_cong,
                    status_cong: conge.status_cong,
                    date_trait_cong: conge.date_trait_cong.map(|dt| Local.from_local_datetime(&dt).unwrap()),
                    employe: employe.map(|empl| UserInfo {
                        n_matricule: empl.n_matricule,
                        id_dep: empl.id_dep,
                        nom_empl: empl.nom_empl,
                        prenom_empl: empl.prenom_empl,
                        email_empl: empl.email_empl,
                        role: empl.role,
                    }),
                    departement: departement.map(|dept| DepartementInfo {
                        code_dep: dept.code_dep,
                        nom_dep: dept.nom_dep,
                    })
                })
            }
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ApiResponse::new(
        200,
        true,
        "All conges with employee Information".to_string(),
        Some(conge_models),
    ))
}


#[put("/approve_conge/{id_cong}")]
pub async fn approve_conge(
    app_state: web::Data<app_state::AppState>,
    id_cong: web::Path<String>,
) -> Result<ApiResponse<String>, ApiResponse<()>> {
    let id_cong = id_cong.clone();
    let conges = entity::conge::Entity::find_by_id(id_cong.clone())
        .one(&app_state.db)
        .await
        .map_err(|e| {
            println!("Error: {}", e);
            ApiResponse::new(500, false, e.to_string(), None)
        })?;
    
    let mut conge = match conges {
        Some(conge) => conge.into_active_model(),
        None => {
            return Ok(ApiResponse::new(200, false, "conge not found".to_string(), None));
        }
    };

    let new_status = match conge.status_cong.as_ref() {
        s if s.contains("RH") => "Approuver".to_string(),
        s if s.contains("DTP") => "En attente RH".to_string(),
        _ => {
            return Ok(ApiResponse::new(200, false, "Status actuel invalide".to_string(), None));
        }
    };

    conge.status_cong = Set(new_status);

    conge
        .update(&app_state.db)
        .await
        .map_err(|err| {
            println!("Error: {}", err);
            ApiResponse::new(500, false, err.to_string(), None)
        })?;

    Ok(ApiResponse::new(
        200, 
        true, 
        "Update conge Successfully".to_string(), 
        None
    ))
}

#[put("/decline_conge/{id_cong}")]
pub async fn decline_conge(
    app_state: web::Data<app_state::AppState>,
    id_cong: web::Path<String>,
) -> Result<ApiResponse<String>, ApiResponse<()>> {
    let id_cong = id_cong.clone();
    let conges = entity::conge::Entity::find_by_id(id_cong.clone())
        .one(&app_state.db)
        .await
        .map_err(|e| {
            println!("Error: {}", e);
            ApiResponse::new(500, false, e.to_string(), None)
        })?;
    
    let mut conge = match conges {
        Some(conge) => conge.into_active_model(),
        None => {
            return Ok(ApiResponse::new(200, false, "conge not found".to_string(), None));
        }
    };

    let solde_result = entity::solde::Entity::find()
        .filter(entity::solde::Column::IdEmpl.eq(conge.id_empl.as_ref()))
        .order_by_desc(entity::solde::Column::AnneeSld)
        .order_by_desc(entity::solde::Column::MoisSld)
        .one(&app_state.db)
        .await
        .map_err(|err| {
            println!("Error: {}", err);
            ApiResponse::new(500, false, err.to_string(), None)
        })?;

    let mut solde = match solde_result {
        Some(solde) => solde.into_active_model(),
        None => {
            return Ok(ApiResponse::new(200, false, "solde not found".to_string(), None));
        }
    };

    let nb_jours_f64 = conge.nb_jour_cong.as_ref().to_f64().unwrap().to_string();
    let nb_jours_decimal = Decimal::from_str(&nb_jours_f64)
        .map_err(|err| {
            println!("Error: {}", err);
            ApiResponse::new(500, false, err.to_string(), None)
        })?;
    
    let new_j_pris_sld = solde.j_pris_sld.as_ref() - nb_jours_decimal;
    let new_j_restant_sld = solde.j_reste_sld.as_ref() + nb_jours_decimal;

    solde.j_pris_sld = Set(new_j_pris_sld);
    solde.j_reste_sld = Set(new_j_restant_sld);

    solde
        .update(&app_state.db)
        .await
        .map_err(|err| {
            println!("Error: {}", err);
            ApiResponse::new(500, false, err.to_string(), None)
        })?;

    let new_status = "Refuser".to_string();

    conge.status_cong = Set(new_status);

    conge
        .update(&app_state.db)
        .await
        .map_err(|err| {
            println!("Error: {}", err);
            ApiResponse::new(500, false, err.to_string(), None)
        })?;

    Ok(ApiResponse::new(
        200, 
        true, 
        "Decline conge Successfully".to_string(), 
        None
    ))
}