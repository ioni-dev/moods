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
use tracing::{debug, info, instrument};
use uuid::Uuid;
use validator::Validate;

#[instrument[skip(repository)]]
pub async fn get_all_contacts(
    user: AuthenticatedUser,
    repository: ContactRepository,
) -> AppResponse {
    let contacts = repository.get_all(user.0).await?;
    // .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(contacts))
}

#[instrument(skip(contact, repository))]
pub async fn create_contact(
    contact: Json<NewContact>,
    repository: ContactRepository,
) -> AppResponse {
    match contact.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("first_name") {
                format!(
                    "Invalid first name, \"{}\" is too short.",
                    contact.first_name
                )
            } else if error_map.contains_key("last_name") {
                format!("Invalid last name, \"{}\" is to short", contact.last_name)
            } else if error_map.contains_key("company_name") {
                format!("Invalid name, \"{:?}\" is to short", contact.company_name)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

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
                (Some(db::UNIQUE_VIOLATION_CODE), Some("cell_phone_number")) => {
                    AppError::INVALID_INPUT.message("Phone number already exists.".to_string())
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

#[instrument[skip(repository)]]
pub async fn update_contact(
    user: AuthenticatedUser,
    contact: Json<UpdateContact>,
    id_contact: String,
    repository: ContactRepository,
) -> AppResponse {
    match contact.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("first_name") {
                format!("Invalid name, too short",)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

    let contact = repository.update_contact(id_contact, contact.0).await?;

    Ok(HttpResponse::Ok().json(contact))
}

#[instrument[skip(repository)]]
pub async fn get_contact(
    user: AuthenticatedUser,
    id_contact: String,
    repository: ContactRepository,
) -> AppResponse {
    let contact = repository
        .find_by_id(user.0, id_contact)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(contact))
}
