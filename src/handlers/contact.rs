use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    config::crypto::CryptoService,
    db,
    db::contact::ContactRepository,
    errors::AppError,
    models::contact::{Contact, NewContact, UpdateContact},
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use color_eyre::Result;
use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, instrument, info};
use validator::Validate;
use uuid::Uuid;

#[instrument[skip(repository)]]
pub async fn get_all_contacts(
    user: AuthenticatedUser,
    repository: ContactRepository,
) -> AppResponse {
    let contacts = repository
        .get_all(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(contacts))
}

#[instrument(skip(contact, repository))]
pub async fn create_contact (
    contact: Json<NewContact>,
    repository: ContactRepository,
) -> AppResponse {
    info!("testing contact params" );
    // match contact.validate() {
    //     Ok(_) => Ok(()),
    //     Err(errors) => {
    //         let error_map = errors.field_errors();

    //         let message = if error_map.contains_key("first_name") {
    //             format!("Invalid first name, \"{}\" is too short.", contact.first_name)
    //         } else if error_map.contains_key("last_name") {
    //             format!("Invalid last name, \"{}\" is to short", contact.last_name)
    //         } else if error_map.contains_key("description") {
    //             format!("Invalid description, \"{}\" is to short", contact.description)
    //         } else if error_map.contains_key("company"){
    //             format!("Invalid description, \"{}\" is to short", contact.company)
    //         } else {
    //             "Invalid input.".to_string()
    //         };

    //         Err(AppError::INVALID_INPUT.message(message));
    //     }
    // }?;

    let result: Result<Contact> = repository.create(contact.0).await;
    match result {
        Ok(contact) => Ok(HttpResponse::Ok().json(contact)),
        Err(error) => {
            let pg_error: &PgError =
                error
                    .root_cause()
                    .downcast_ref::<PgError>()
                    .ok_or_else(|| {
                        debug!("Error creating contact. {:?}", error);
                        AppError::INTERNAL_ERROR
                    })?;

            let error = match (pg_error.code(), pg_error.column_name()) {
                (Some(db::UNIQUE_VIOLATION_CODE), Some("email")) => {
                    AppError::INVALID_INPUT.message("Email address already exists.".to_string())
                }
                (Some(db::UNIQUE_VIOLATION_CODE), Some("name")) => {
                    AppError::INVALID_INPUT.message("name already exists.".to_string())
                }
                (Some(db::UNIQUE_VIOLATION_CODE), None) => {
                    AppError::INVALID_INPUT.message("name or email already exists.".to_string())
                }
                _ => {
                    debug!("Error creating contact. {:?}", pg_error);
                    AppError::INTERNAL_ERROR.into()
                }
            };
            Err(error)
        }
    }

}
