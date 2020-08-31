use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    config::crypto::CryptoService,
    db,
    ,
    errors::AppError,
    models::contact::{Contact, NewContact, UpdateContact},
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use color_eyre::Result;
// use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, instrument};

#[instrument[skip(repository)]]
pub async fn get_all_contacts(
    user: AuthenticatedUser,
    repository: contactRepository,
) -> AppResponse {
    let contacts = repository
        .get_all(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(contacts))
}

pub async fn create_contact(
    contact: Json<NewContact>,
    repository: ContactRepository,
    contact: Contact,
) -> AppResponse {
    match contact.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("name") {
                format!("Invalid title. \"{}\" is too short.", contact.first_name)
            } else if error_map.contains_key("content") {
                format!("Invalid content \"{}\" is to short", contact.last_name)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message));
        }
    }?;
}
