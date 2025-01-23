use actix_web::test;
use chrono::{Local, NaiveDate};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;

use crate::{
    routes::handlers::conge_handlers::{all_conge, CongeWithEmploye},
    schema::conge::Entity as CongeEntity,
    schema::employe::Entity as EmployeEntity,
    utils::app_state::AppState,
};

#[actix_rt::test]
async fn test_all_conge_returns_empty_list_when_no_conges() {
    // Arrange
    let app_state = AppState::new();
    let db = app_state.db.get_database().await.unwrap();

    // Act
    let result = all_conge(web::Data::new(app_state)).await;

    // Assert
    match result {
        Ok(api_response) => {
            assert_eq!(api_response.status_code, 200);
            assert!(api_response.success);
            assert_eq!(api_response.message, "All conges with employee Information");
            assert_eq!(api_response.data.unwrap(), vec![]);
        }
        Err(_) => panic!("Expected Ok response"),
    }
}use actix_web::test;
use chrono::{Local, NaiveDate};
use sea_orm::prelude::Decimal;
use serde_json::json;

use crate::{
    routes::handlers::{conge_handlers::all_conge, employe_handlers::UserInfo},
    utils::api_response::ApiResponse,
};

#[actix_rt::test]
async fn test_all_conge_with_employee_info() {
    // Arrange
    let app_state = create_app_state_with_mock_db();
    let mut req = test::TestRequest::get().uri("/all_conge").data(app_state);

    // Act
    let response = all_conge(req.extract().await.unwrap()).await.unwrap();

    // Assert
    let expected_response = ApiResponse::new(
        200,
        true,
        "All conges with employee Information".to_string(),
        Some(vec![CongeWithEmploye {
            date_dmd_cong: Local::now(),
            motif_cong: "Vacation".to_string(),
            date_deb_cong: NaiveDate::from_ymd(2022, 1, 1),
            date_fin_cong: NaiveDate::from_ymd(2022, 1, 7),
            nb_jour_cong: Decimal::from_f64(7.0).unwrap(),
            status_cong: "Pending".to_string(),
            date_trait_cong: None,
            employe: vec![UserInfo {
                n_matricule: "EMP001".to_string(),
                id_dep: 1,
                nom_empl: "Doe".to_string(),
                prenom_empl: "John".to_string(),
                email_empl: "johndoe@example.com".to_string(),
                role: "Employee".to_string(),
            }],
        }]),
    );

    assert_eq!(response, expected_response);
}

// Mock implementation for app_state and database operations
fn create_app_state_with_mock_db() -> web::Data<app_state::AppState> {
    // Create mock app_state with mocked database operations
    let db = MockDatabase::new();
    let app_state = app_state::AppState { db };
    web::Data::new(app_state)
}

struct MockDatabase;

impl MockDatabase {
    fn new() -> Self {
        // Initialize mock database
        MockDatabase
    }
}

// Mock implementation for database operations
impl sea_orm::DatabaseConnection for MockDatabase {
    type Database = MockDatabase;
    type Error = std::io::Error;

    fn fetch_one<'a, T>(&'a self, query: Query<'_, Self::Database>) -> Result<Option<T>, Self::Error>
    where
        T: FromRow<'a, Self::Database>,
    {
        // Mock implementation for fetching one record
        Ok(Some(T::from_row(&[]))) // Return mock data
    }

    fn fetch_all<'a, T>(&'a self, query: Query<'_, Self::Database>) -> Result<Vec<T>, Self::Error>
    where
        T: FromRow<'a, Self::Database>,
    {
        // Mock implementation for fetching all records
        Ok(vec![T::from_row(&[])]) // Return mock data
    }

    // Implement other database operations as needed
}#[actix_rt::test]
async fn test_all_conge_returns_correct_conge_info_for_specific_employee() {
    // Arrange
    let app_state = create_app_state_with_mock_db();
    let mut req = test::TestRequest::get().uri("/all_conge").data(app_state);

    // Act
    let response = all_conge(req.extract().await.unwrap()).await.unwrap();

    // Assert
    let expected_conge_info = CongeWithEmploye {
        date_dmd_cong: Local::now(),
        motif_cong: "Vacation".to_string(),
        date_deb_cong: NaiveDate::from_ymd(2022, 1, 1),
        date_fin_cong: NaiveDate::from_ymd(2022, 1, 7),
        nb_jour_cong: Decimal::from_f64(7.0).unwrap(),
        status_cong: "Pending".to_string(),
        date_trait_cong: None,
        employe: vec![UserInfo {
            n_matricule: "EMP001".to_string(),
            id_dep: 1,
            nom_empl: "Doe".to_string(),
            prenom_empl: "John".to_string(),
            email_empl: "johndoe@example.com".to_string(),
            role: "Employee".to_string(),
        }],
    };

    assert_eq!(response.status_code, 200);
    assert!(response.success);
    assert_eq!(response.message, "All conges with employee Information");
    assert_eq!(response.data.unwrap().len(), 1);
    assert_eq!(response.data.unwrap()[0], expected_conge_info);
}