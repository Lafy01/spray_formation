use actix_web::{
    delete, get, post,
    web::{self},
};
use sea_orm::{
    prelude::Decimal, ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};

use super::conge_handlers::CongePlanning;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithDep {
    pub id_empl: String,
    pub n_matricule: String,
    pub nom_empl: String,
    pub prenom_empl: Option<String>,
    pub email_empl: String,
    pub role: String,
    pub departement: Option<DepartementData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithDepSolde {
    pub id_empl: String,
    pub n_matricule: String,
    pub nom_empl: String,
    pub prenom_empl: Option<String>,
    pub email_empl: String,
    pub role: String,
    pub status: String,
    pub departement: Option<DepartementData>,
    pub solde: Option<SoldeData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningConge {
    pub id_empl: String,
    pub n_matricule: String,
    pub nom_empl: String,
    pub prenom_empl: Option<String>,
    pub email_empl: String,
    pub role: String,
    pub status: String,
    pub departement: Option<DepartementData>,
    pub solde: Option<SoldeData>,
    pub conge: Vec<CongePlanning>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub n_matricule: String,
    pub id_dep: String,
    pub nom_empl: String,
    pub prenom_empl: Option<String>,
    pub email_empl: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepartementData {
    pub code_dep: String,
    pub nom_dep: String,
    pub chef_dep: Option<UserInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepartementInfo {
    pub code_dep: String,
    pub nom_dep: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    id_empl: String,
    n_matricule: String,
    id_dep: String,
    nom_empl: String,
    prenom_empl: Option<String>,
    email_empl: String,
    role: String,
    status: String,
    departement: Option<DepartementData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuperieurData {
    id_empl: String,
    nom_empl: String,
    prenom_empl: Option<String>,
    email_empl: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    id_empl: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoldeData {
    pub j_aqui_sld: Decimal,
    pub j_pris_sld: Decimal,
    pub j_reste_sld: Decimal,
    pub mois_sld: i32,
    pub annee_sld: i32,
}

#[get("/me")]
pub async fn me(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse<UserWithDepSolde>, ApiResponse<()>> {
    let employe = entity::employe::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?
        .ok_or(ApiResponse::new(
            404,
            false,
            "User not found".to_owned(),
            None,
        ))?;

    let departement = entity::departement::Entity::find_by_id(employe.id_dep)
        .one(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

    let chef_dep = match departement {
        Some(ref dept) => entity::employe::Entity::find_by_id(
            dept.chef_dep.clone().unwrap_or_else(|| String::new()),
        )
        .one(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?,
        None => None,
    };

    let solde = entity::solde::Entity::find()
        .filter(entity::solde::Column::IdEmpl.eq(employe.id_empl.clone()))
        .order_by_desc(entity::solde::Column::AnneeSld)
        .order_by_desc(entity::solde::Column::MoisSld)
        .one(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

            let today = chrono::Local::now().date_naive();

            let status_empl = if let Some(_conge) = entity::conge::Entity::find()
                .filter(entity::conge::Column::IdEmpl.eq(employe.id_empl.clone()))
                .filter(entity::conge::Column::DateDebCong.lte(today))
                .filter(entity::conge::Column::DateFinCong.gte(today))
                .one(&app_state.db)
                .await
                .map_err(|err| {
                    println!("{:?}", err);
                    ApiResponse::new(500, false, err.to_string(), None)
                })?
                {
                    "innactif".to_string()
                } else {
                    "actif".to_string()
                };

    let employe_model = UserWithDepSolde {
        id_empl: employe.id_empl.clone(),
        n_matricule: employe.n_matricule.clone(),
        nom_empl: employe.nom_empl.clone(),
        prenom_empl: employe.prenom_empl.clone(),
        email_empl: employe.email_empl.clone(),
        role: employe.role.clone(),
        status: status_empl,
        departement: departement.map(|dep| DepartementData {
            code_dep: dep.code_dep.clone(),
            nom_dep: dep.nom_dep.clone(),
            chef_dep: chef_dep
                .map(|chef| {
                    if chef.n_matricule == employe.n_matricule {
                        Some(UserInfo {
                            n_matricule: "me".to_string(),
                            id_dep: chef.id_dep,
                            nom_empl: chef.nom_empl,
                            prenom_empl: chef.prenom_empl,
                            email_empl: chef.email_empl,
                            role: chef.role,
                        })
                    } else {
                        Some(UserInfo {
                            n_matricule: chef.n_matricule,
                            id_dep: chef.id_dep,
                            nom_empl: chef.nom_empl,
                            prenom_empl: chef.prenom_empl,
                            email_empl: chef.email_empl,
                            role: chef.role,
                        })
                    }
                })
                .flatten(),
        }),
        solde: solde.map(|sld| SoldeData {
            j_aqui_sld: sld.j_aqui_sld,
            j_pris_sld: sld.j_pris_sld,
            j_reste_sld: sld.j_reste_sld,
            mois_sld: sld.mois_sld,
            annee_sld: sld.annee_sld,
        }),
    };

    Ok(ApiResponse::new(
        200,
        true,
        "User found".to_string(),
        Some(employe_model),
    ))
}

#[get("/all_user")]
pub async fn all_user(
    app_state: web::Data<app_state::AppState>,
    claims: Claims,
) -> Result<ApiResponse<Vec<UserWithDepSolde>>, ApiResponse<()>> {
    let employees = entity::employe::Entity::find()
        .filter(entity::employe::Column::IdEmpl.ne(claims.id))
        .all(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

    let employee_models = futures::future::join_all(employees.into_iter().map(|employee| {
        let app_state = app_state.clone();
        async move {
            let departement = entity::departement::Entity::find_by_id(employee.id_dep)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;
            let chef_dep = match &departement {
                Some(dep) => {
                    entity::employe::Entity::find_by_id(dep.chef_dep.clone().unwrap_or_default())
                        .one(&app_state.db)
                        .await
                        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?
                }
                None => None,
            };

            let solde = entity::solde::Entity::find()
                .filter(entity::solde::Column::IdEmpl.eq(employee.id_empl.clone()))
                .order_by_desc(entity::solde::Column::AnneeSld)
                .order_by_desc(entity::solde::Column::MoisSld)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

            let today = chrono::Local::now().date_naive();

            let status_empl = if let Some(_conge) = entity::conge::Entity::find()
                .filter(entity::conge::Column::IdEmpl.eq(employee.id_empl.clone()))
                .filter(entity::conge::Column::DateDebCong.lte(today))
                .filter(entity::conge::Column::DateFinCong.gte(today))
                .one(&app_state.db)
                .await
                .map_err(|err| {
                    println!("{:?}", err);
                    ApiResponse::new(500, false, err.to_string(), None)
                })?
                {
                    "innactif".to_string()
                } else {
                    "actif".to_string()
                };


            Ok(UserWithDepSolde {
                id_empl: employee.id_empl,
                n_matricule: employee.n_matricule,
                nom_empl: employee.nom_empl,
                prenom_empl: employee.prenom_empl,
                email_empl: employee.email_empl,
                role: employee.role,
                status: status_empl,
                departement: departement.map(|dep| DepartementData {
                    code_dep: dep.code_dep,
                    nom_dep: dep.nom_dep,
                    chef_dep: chef_dep.map(|chef| UserInfo {
                        n_matricule: chef.n_matricule,
                        id_dep: chef.id_dep,
                        nom_empl: chef.nom_empl,
                        prenom_empl: chef.prenom_empl,
                        email_empl: chef.email_empl,
                        role: chef.role,
                    }),
                }),
                solde: solde.map(|sld| SoldeData {
                    j_aqui_sld: sld.j_aqui_sld,
                    j_pris_sld: sld.j_pris_sld,
                    j_reste_sld: sld.j_reste_sld,
                    mois_sld: sld.mois_sld,
                    annee_sld: sld.annee_sld,
                }),
            })
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?;

    Ok(ApiResponse::new(
        200,
        true,
        "All user fetched successfully".to_string(),
        Some(employee_models),
    ))
}

#[get("/all_employe")]
pub async fn all_employe(
    app_state: web::Data<app_state::AppState>,
    claims: Claims,
) -> Result<ApiResponse<Vec<UserWithDepSolde>>, ApiResponse<()>> {
    let employees = entity::employe::Entity::find()
        .filter(entity::employe::Column::IdEmpl.ne(claims.id))
        .filter(entity::employe::Column::Role.ne("admin"))
        .filter(entity::employe::Column::Role.ne("rh"))
        .all(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

    let employee_models = futures::future::join_all(employees.into_iter().map(|employee| {
        let app_state = app_state.clone();
        async move {
            let departement = entity::departement::Entity::find_by_id(employee.id_dep)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;
            let chef_dep = match &departement {
                Some(dep) => {
                    entity::employe::Entity::find_by_id(dep.chef_dep.clone().unwrap_or_default())
                        .one(&app_state.db)
                        .await
                        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?
                }
                None => None,
            };

            let solde = entity::solde::Entity::find()
                .filter(entity::solde::Column::IdEmpl.eq(employee.id_empl.clone()))
                .order_by_desc(entity::solde::Column::AnneeSld)
                .order_by_desc(entity::solde::Column::MoisSld)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

            let today = chrono::Local::now().date_naive();

            let status_empl = if let Some(_conge) = entity::conge::Entity::find()
                .filter(entity::conge::Column::IdEmpl.eq(employee.id_empl.clone()))
                .filter(entity::conge::Column::DateDebCong.lte(today))
                .filter(entity::conge::Column::DateFinCong.gte(today))
                .one(&app_state.db)
                .await
                .map_err(|err| {
                    println!("{:?}", err);
                    ApiResponse::new(500, false, err.to_string(), None)
                })?
                {
                    "innactif".to_string()
                } else {
                    "actif".to_string()
                };

            Ok(UserWithDepSolde {
                id_empl: employee.id_empl,
                n_matricule: employee.n_matricule,
                nom_empl: employee.nom_empl,
                prenom_empl: employee.prenom_empl,
                email_empl: employee.email_empl,
                role: employee.role,
                status: status_empl,
                departement: departement.map(|dep| DepartementData {
                    code_dep: dep.code_dep,
                    nom_dep: dep.nom_dep,
                    chef_dep: chef_dep.map(|chef| UserInfo {
                        n_matricule: chef.n_matricule,
                        id_dep: chef.id_dep,
                        nom_empl: chef.nom_empl,
                        prenom_empl: chef.prenom_empl,
                        email_empl: chef.email_empl,
                        role: chef.role,
                    }),
                }),
                solde: solde.map(|sld| SoldeData {
                    j_aqui_sld: sld.j_aqui_sld,
                    j_pris_sld: sld.j_pris_sld,
                    j_reste_sld: sld.j_reste_sld,
                    mois_sld: sld.mois_sld,
                    annee_sld: sld.annee_sld,
                }),
            })
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?;

    Ok(ApiResponse::new(
        200,
        true,
        "All employees fetched successfully".to_string(),
        Some(employee_models),
    ))
}

#[get("/all_my_employe")]
pub async fn all_my_employe(
    app_state: web::Data<app_state::AppState>,
    claims: Claims,
) -> Result<ApiResponse<Vec<UserWithDepSolde>>, ApiResponse<()>> {
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
                true,
                "No Department Found".to_string(),
                Some(vec![]),
            ));
        }
    };

    let employees = entity::employe::Entity::find()
        .filter(entity::employe::Column::IdDep.eq(chef_departement_id))
        .filter(entity::employe::Column::IdEmpl.ne(claims.id))
        .filter(entity::employe::Column::Role.eq("employe"))
        .all(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

    let employee_models = futures::future::join_all(employees.into_iter().map(|employee| {
        let app_state = app_state.clone();
        async move {
            let departement = entity::departement::Entity::find_by_id(employee.id_dep)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;
            let chef_dep = match &departement {
                Some(dep) => {
                    entity::employe::Entity::find_by_id(dep.chef_dep.clone().unwrap_or_default())
                        .one(&app_state.db)
                        .await
                        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?
                }
                None => None,
            };

            let solde = entity::solde::Entity::find()
                .filter(entity::solde::Column::IdEmpl.eq(employee.id_empl.clone()))
                .order_by_desc(entity::solde::Column::AnneeSld)
                .order_by_desc(entity::solde::Column::MoisSld)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

            let today = chrono::Local::now().date_naive();

            let status_empl = if let Some(_conge) = entity::conge::Entity::find()
                .filter(entity::conge::Column::IdEmpl.eq(employee.id_empl.clone()))
                .filter(entity::conge::Column::DateDebCong.lte(today))
                .filter(entity::conge::Column::DateFinCong.gte(today))
                .one(&app_state.db)
                .await
                .map_err(|err| {
                    println!("{:?}", err);
                    ApiResponse::new(500, false, err.to_string(), None)
                })?
                {
                    "innactif".to_string()
                } else {
                    "actif".to_string()
                };

            Ok(UserWithDepSolde {
                id_empl: employee.id_empl,
                n_matricule: employee.n_matricule,
                nom_empl: employee.nom_empl,
                prenom_empl: employee.prenom_empl,
                email_empl: employee.email_empl,
                role: employee.role,
                status: status_empl,
                departement: departement.map(|dep| DepartementData {
                    code_dep: dep.code_dep,
                    nom_dep: dep.nom_dep,
                    chef_dep: chef_dep.map(|chef| UserInfo {
                        n_matricule: chef.n_matricule,
                        id_dep: chef.id_dep,
                        nom_empl: chef.nom_empl,
                        prenom_empl: chef.prenom_empl,
                        email_empl: chef.email_empl,
                        role: chef.role,
                    }),
                }),
                solde: solde.map(|sld| SoldeData {
                    j_aqui_sld: sld.j_aqui_sld,
                    j_pris_sld: sld.j_pris_sld,
                    j_reste_sld: sld.j_reste_sld,
                    mois_sld: sld.mois_sld,
                    annee_sld: sld.annee_sld,
                }),
            })
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?;

    Ok(ApiResponse::new(
        200,
        true,
        "All employees fetched successfully".to_string(),
        Some(employee_models),
    ))
}

#[get("/all_employe_planning")]
pub async fn all_employe_planning(
    app_state: web::Data<app_state::AppState>,
    claims: Claims,
) -> Result<ApiResponse<Vec<PlanningConge>>, ApiResponse<()>> {
    let employees = entity::employe::Entity::find()
        .filter(entity::employe::Column::IdEmpl.ne(claims.id))
        .filter(entity::employe::Column::Role.ne("admin"))
        .filter(entity::employe::Column::Role.ne("rh"))
        .all(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

    let employee_models = futures::future::join_all(employees.into_iter().map(|employee| {
        let app_state = app_state.clone();
        async move {
            let departement = entity::departement::Entity::find_by_id(employee.id_dep)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;
            let chef_dep = match &departement {
                Some(dep) => {
                    entity::employe::Entity::find_by_id(dep.chef_dep.clone().unwrap_or_default())
                        .one(&app_state.db)
                        .await
                        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?
                }
                None => None,
            };

            let solde = entity::solde::Entity::find()
                .filter(entity::solde::Column::IdEmpl.eq(employee.id_empl.clone()))
                .order_by_desc(entity::solde::Column::AnneeSld)
                .order_by_desc(entity::solde::Column::MoisSld)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

            let conges = entity::conge::Entity::find()
                .filter(entity::conge::Column::IdEmpl.eq(employee.id_empl.clone()))
                .all(&app_state.db)
                .await
                .map_err(|err| {
                    println!("{:?}", err);
                    ApiResponse::new(500, false, err.to_string(), None)
                })?;

            let conge_planning: Vec<CongePlanning> = conges
                .into_iter()
                .map(|conge| CongePlanning {
                    id_cong: conge.id_cong,
                    motif_cong: conge.motif_cong,
                    date_dmd_cong: conge.date_dmd_cong,
                    date_deb_cong: conge.date_deb_cong,
                    date_fin_cong: conge.date_fin_cong,
                    nb_jour_cong: conge.nb_jour_cong,
                    status_cong: conge.status_cong,
                    date_trait_cong: conge.date_trait_cong,
                })
                .collect();

            let today = chrono::Local::now().date_naive();

            let status_empl = if let Some(_conge) = entity::conge::Entity::find()
                .filter(entity::conge::Column::IdEmpl.eq(employee.id_empl.clone()))
                .filter(entity::conge::Column::DateDebCong.lte(today))
                .filter(entity::conge::Column::DateFinCong.gte(today))
                .one(&app_state.db)
                .await
                .map_err(|err| {
                    println!("{:?}", err);
                    ApiResponse::new(500, false, err.to_string(), None)
                })?
                {
                    "innactif".to_string()
                } else {
                    "actif".to_string()
                };


            Ok(PlanningConge {
                id_empl: employee.id_empl,
                n_matricule: employee.n_matricule,
                nom_empl: employee.nom_empl,
                prenom_empl: employee.prenom_empl,
                email_empl: employee.email_empl,
                role: employee.role,
                status: status_empl,
                departement: departement.map(|dep| DepartementData {
                    code_dep: dep.code_dep,
                    nom_dep: dep.nom_dep,
                    chef_dep: chef_dep.map(|chef| UserInfo {
                        n_matricule: chef.n_matricule,
                        id_dep: chef.id_dep,
                        nom_empl: chef.nom_empl,
                        prenom_empl: chef.prenom_empl,
                        email_empl: chef.email_empl,
                        role: chef.role,
                    }),
                }),
                solde: solde.map(|sld| SoldeData {
                    j_aqui_sld: sld.j_aqui_sld,
                    j_pris_sld: sld.j_pris_sld,
                    j_reste_sld: sld.j_reste_sld,
                    mois_sld: sld.mois_sld,
                    annee_sld: sld.annee_sld,
                }),
                conge: conge_planning,
            })
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?;

    Ok(ApiResponse::new(
        200,
        true,
        "All employees fetched successfully".to_string(),
        Some(employee_models),
    ))
}

#[get("/all_my_employe_planning")]
pub async fn all_my_employe_planning(
    app_state: web::Data<app_state::AppState>,
    claims: Claims,
) -> Result<ApiResponse<Vec<PlanningConge>>, ApiResponse<()>> {
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
                true,
                "No Department Found".to_string(),
                Some(vec![]),
            ));
        }
    };

    let employees = entity::employe::Entity::find()
        .filter(entity::employe::Column::IdDep.eq(chef_departement_id))
        .filter(entity::employe::Column::IdEmpl.ne(claims.id))
        .filter(entity::employe::Column::Role.eq("employe"))
        .all(&app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

    let employee_models = futures::future::join_all(employees.into_iter().map(|employee| {
        let app_state = app_state.clone();
        async move {
            let departement = entity::departement::Entity::find_by_id(employee.id_dep)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;
            let chef_dep = match &departement {
                Some(dep) => {
                    entity::employe::Entity::find_by_id(dep.chef_dep.clone().unwrap_or_default())
                        .one(&app_state.db)
                        .await
                        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?
                }
                None => None,
            };

            let solde = entity::solde::Entity::find()
                .filter(entity::solde::Column::IdEmpl.eq(employee.id_empl.clone()))
                .order_by_desc(entity::solde::Column::AnneeSld)
                .order_by_desc(entity::solde::Column::MoisSld)
                .one(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

            let conges = entity::conge::Entity::find()
                .filter(entity::conge::Column::IdEmpl.eq(employee.id_empl.clone()))
                .all(&app_state.db)
                .await
                .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;

            let conge_planning: Vec<CongePlanning> = conges
                .into_iter()
                .map(|conge| CongePlanning {
                    id_cong: conge.id_cong,
                    motif_cong: conge.motif_cong,
                    date_dmd_cong: conge.date_dmd_cong,
                    date_deb_cong: conge.date_deb_cong,
                    date_fin_cong: conge.date_fin_cong,
                    nb_jour_cong: conge.nb_jour_cong,
                    status_cong: conge.status_cong,
                    date_trait_cong: conge.date_trait_cong,
                })
                .collect();

            let today = chrono::Local::now().date_naive();

            let status_empl = if let Some(_conge) = entity::conge::Entity::find()
                .filter(entity::conge::Column::IdEmpl.eq(employee.id_empl.clone()))
                .filter(entity::conge::Column::DateDebCong.lte(today))
                .filter(entity::conge::Column::DateFinCong.gte(today))
                .one(&app_state.db)
                .await
                .map_err(|err| {
                    println!("{:?}", err);
                    ApiResponse::new(500, false, err.to_string(), None)
                })?
                {
                    "innactif".to_string()
                } else {
                    "actif".to_string()
                };

            Ok(PlanningConge {
                id_empl: employee.id_empl,
                n_matricule: employee.n_matricule,
                nom_empl: employee.nom_empl,
                prenom_empl: employee.prenom_empl,
                email_empl: employee.email_empl,
                role: employee.role,
                status: status_empl,
                departement: departement.map(|dep| DepartementData {
                    code_dep: dep.code_dep,
                    nom_dep: dep.nom_dep,
                    chef_dep: chef_dep.map(|chef| UserInfo {
                        n_matricule: chef.n_matricule,
                        id_dep: chef.id_dep,
                        nom_empl: chef.nom_empl,
                        prenom_empl: chef.prenom_empl,
                        email_empl: chef.email_empl,
                        role: chef.role,
                    }),
                }),
                solde: solde.map(|sld| SoldeData {
                    j_aqui_sld: sld.j_aqui_sld,
                    j_pris_sld: sld.j_pris_sld,
                    j_reste_sld: sld.j_reste_sld,
                    mois_sld: sld.mois_sld,
                    annee_sld: sld.annee_sld,
                }),
                conge: conge_planning,
            })
        }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?;

    Ok(ApiResponse::new(
        200,
        true,
        "All employees fetched successfully".to_string(),
        Some(employee_models),
    ))
}

#[post("/update_employe/{id_empl}")]
pub async fn update_employe(
    app_state: web::Data<app_state::AppState>,
    path: web::Path<UserId>,
    employe_json: web::Json<UserInfo>,
) -> Result<ApiResponse<UserModel>, ApiResponse<()>> {
    let id_empl = path.id_empl.clone();

    let user_model = match entity::employe::Entity::find_by_id(id_empl)
        .one(&app_state.db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok(ApiResponse::new(
                404,
                false,
                "User not found".to_owned(),
                None,
            ))
        }
        Err(e) => return Ok(ApiResponse::new(500, false, e.to_string(), None)),
    };

    let mut user_model: entity::employe::ActiveModel = user_model.into();

    user_model.n_matricule = Set(employe_json.n_matricule.clone());
    user_model.id_dep = Set(employe_json.id_dep.clone());
    user_model.nom_empl = Set(employe_json.nom_empl.clone());
    user_model.prenom_empl = Set(employe_json.prenom_empl.clone());
    user_model.email_empl = Set(employe_json.email_empl.clone());
    user_model.role = Set(employe_json.role.to_lowercase().clone());

    let updated_user_model = user_model
        .update(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, false, err.to_string(), None))?;

    let user_data = UserModel {
        id_empl: updated_user_model.id_empl,
        n_matricule: updated_user_model.n_matricule,
        id_dep: updated_user_model.id_dep,
        nom_empl: updated_user_model.nom_empl,
        prenom_empl: updated_user_model.prenom_empl,
        email_empl: updated_user_model.email_empl,
        role: updated_user_model.role,
        status: updated_user_model.status,
        departement: None,
    };

    Ok(ApiResponse::new(
        200,
        true,
        "User updated successfully".to_owned(),
        Some(user_data),
    ))
}

#[delete("/delete_user/{id_empl}")]
pub async fn delete_user(
    app_state: web::Data<app_state::AppState>,
    path: web::Path<UserId>,
) -> Result<ApiResponse<()>, ApiResponse<()>> {
    let id_empl = path.id_empl.clone();
    let user_model = match entity::employe::Entity::find_by_id(id_empl)
        .one(&app_state.db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok(ApiResponse::new(
                404,
                false,
                "User not found".to_owned(),
                None,
            ))
        }
        Err(e) => return Ok(ApiResponse::new(500, false, e.to_string(), None)),
    };
    entity::employe::Model::delete(user_model, &app_state.db)
        .await
        .map_err(|e| ApiResponse::new(500, false, e.to_string(), None))?;
    Ok(ApiResponse::new(
        200,
        true,
        "User deleted successfully".to_owned(),
        None,
    ))
}