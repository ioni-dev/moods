use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    config::crypto::CryptoService,
    db,
    db::event::EventRepository,
    errors::AppError,
    models::event::{Event, NewEvent, UpdateEvent},
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use color_eyre::Result;
// use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, instrument};
use validator::Validate;

// #[instrument(skip(event, repository, crypto_service))]
// pub async fn create_event(
//     event: Json<NewEvent>,
//     repository: EventRepository,
//     crypto_service: Data<CryptoService>,
// ) -> AppResponse {
//     match event.validate() {
//         Ok(_) => Ok(()),
//         Err(errors) => {
//             let error_map = errors.field_errors();

//             let message = if error_map.contains_key("name") {
//                 format!("Invalid user name. \"{}\" is too short.", event.name)
//             } else if error_map.contains_key("email") {
//                 format!("Invalid email address \"{}\"", event.description)
//             } else if error_map.contains_key("password") {
//                 "Invalid password. Too short".to_string()
//             } else {
//                 "Invalid input.".to_string()
//             };

//             Err(AppError::INVALID_INPUT.message(message))
//         }
//     }?;

//     let result: Result<Event> = repository.create(event.0, crypto_service.as_ref()).await;

//     match result {
//         Ok(user) => Ok(HttpResponse::Ok().json(user)),
//         Err(error) => {
//             let pg_error: &PgError =
//                 error
//                     .root_cause()
//                     .downcast_ref::<PgError>()
//                     .ok_or_else(|| {
//                         debug!("Error creating user. {:?}", error);
//                         AppError::INTERNAL_ERROR
//                     })?;

//             let error = match (pg_error.code(), pg_error.column_name()) {
//                 (Some(db::UNIQUE_VIOLATION_CODE), Some("email")) => {
//                     AppError::INVALID_INPUT.message("Email address already exists.".to_string())
//                 }
//                 (Some(db::UNIQUE_VIOLATION_CODE), Some("name")) => {
//                     AppError::INVALID_INPUT.message("name already exists.".to_string())
//                 }
//                 (Some(db::UNIQUE_VIOLATION_CODE), None) => {
//                     AppError::INVALID_INPUT.message("name or email already exists.".to_string())
//                 }
//                 _ => {
//                     debug!("Error creating user. {:?}", pg_error);
//                     AppError::INTERNAL_ERROR.into()
//                 }
//             };
//             Err(error)
//         }
//     }
// }



#[instrument[skip(repository)]]
pub async fn get_all_events(user: AuthenticatedUser, repository: EventRepository) -> AppResponse {
    let events = repository
        .get_all(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(events))
}
