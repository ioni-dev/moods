use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    config::crypto::CryptoService,
    db,
    db::appointment::AppointmentRepository,
    errors::AppError,
    models::appointment::{Appointment, NewAppointment, UpdateAppointment},
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use color_eyre::Result;
use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, info, instrument};
use uuid::Uuid;
use validator::Validate;

#[instrument[skip(repository)]]
pub async fn get_all_appointments(
    user: AuthenticatedUser,
    repository: AppointmentRepository,
) -> AppResponse {
    let appointments = repository
        .get_all(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(appointments))
}

#[instrument(skip(appointment, repository, crypto_service))]
pub async fn create_appointment(
    appointment: Json<NewAppointment>,
    repository: AppointmentRepository,
    crypto_service: Data<CryptoService>,
) -> AppResponse {
    match appointment.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("name") {
                format!("Invalid name, \"{}\" is too short.", appointment.name)
            } else if error_map.contains_key("description") {
                format!(
                    "Invalid description, \"{}\" is to short",
                    appointment.description
                )
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

    let result: Result<Appointment> = repository.create(appointment.0).await;
    match result {
        Ok(appointment) => Ok(HttpResponse::Ok().json(appointment)),
        Err(error) => {
            let pg_error: &PgError =
                error
                    .root_cause()
                    .downcast_ref::<PgError>()
                    .ok_or_else(|| {
                        debug!("Error creating appointment. {:?}", error);
                        AppError::INTERNAL_ERROR
                    })?;

            let error = match (pg_error.code(), pg_error.column_name()) {
                (Some(db::UNIQUE_VIOLATION_CODE), Some("name")) => {
                    AppError::INVALID_INPUT.message("Name address already exists.".to_string())
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
