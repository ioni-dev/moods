use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    config::crypto::CryptoService,
    db,
    db::employee::EmployeeRepository,
    errors::AppError,
    models::employee::{Employee, NewEmployee, UpdateEmployee},
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use color_eyre::Result;
use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, instrument};
use uuid::Uuid;
use validator::Validate;

#[instrument[skip(repository)]]
pub async fn get_all_employees(
    user: AuthenticatedUser,
    repository: EmployeeRepository,
) -> AppResponse {
    let employees = repository.get_all(user.0).await?;
    // .ok_or(AppError::INTERNAL_ERROR);

    Ok(HttpResponse::Ok().json(employees))
}

#[instrument(skip(employee, repository, crypto_service))]
pub async fn create_employee(
    id_user: String,
    employee: Json<NewEmployee>,
    repository: EmployeeRepository,
    crypto_service: Data<CryptoService>,
) -> AppResponse {
    match employee.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("name") {
                format!("Invalid name, \"{}\" is too short.", employee.name)
            } else if error_map.contains_key("email") {
                format!("Invalid email, \"{}\" ", employee.email)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

    let result: Result<Employee> = repository
        .create(id_user, employee.0, &crypto_service)
        .await;

    match result {
        Ok(employee) => Ok(HttpResponse::Ok().json(employee)),
        Err(error) => {
            let pg_error: &PgError =
                error
                    .root_cause()
                    .downcast_ref::<PgError>()
                    .ok_or_else(|| {
                        debug!("Error creating employee. {:?}", error);
                        AppError::INTERNAL_ERROR
                    })?;

            let error = match (pg_error.code(), pg_error.column_name()) {
                (Some(db::UNIQUE_VIOLATION_CODE), Some("email")) => {
                    AppError::INVALID_INPUT.message("Email address already exists.".to_string())
                }
                _ => {
                    debug!("Error creating appointment. {:?}", pg_error);
                    AppError::INTERNAL_ERROR.into()
                }
            };
            Err(error)
        }
    }
}

#[instrument[skip(repository)]]
pub async fn update_employee(
    user: AuthenticatedUser,
    employee: Json<UpdateEmployee>,
    id_employee: String,
    repository: EmployeeRepository,
    crypto_service: Data<CryptoService>,
) -> AppResponse {
    match employee.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("name") {
                format!("Invalid name, too short",)
            } else if error_map.contains_key("email") {
                format!("Invalid email, \"{}\" ", employee.email)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

    let employee = repository
        .update_employee(id_employee, employee.0, &crypto_service)
        .await?;

    Ok(HttpResponse::Ok().json(employee))
}

#[instrument[skip(repository)]]
pub async fn get_employee(
    user: AuthenticatedUser,
    id_employee: String,
    repository: EmployeeRepository,
) -> AppResponse {
    let employee = repository
        .find_by_id(id_employee)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(employee))
}
