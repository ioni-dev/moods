use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    config::crypto::CryptoService,
    db,
    db::note::NoteRepository,
    errors::AppError,
    models::note::{NewNote, Note, UpdateNote},
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use color_eyre::Result;
use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, instrument};
use validator::Validate;

#[instrument[skip(repository)]]
pub async fn get_all_notes(user: AuthenticatedUser, repository: NoteRepository) -> AppResponse {
    let notes = repository.get_all(user.0.to_string()).await?;
    // .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(notes))
}

#[instrument(skip(note, repository))]
pub async fn create_note(
    user: AuthenticatedUser,
    note: Json<NewNote>,
    repository: NoteRepository,
) -> AppResponse {
    match note.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("name") {
                format!("Invalid name, \"{}\" is too short.", note.title)
            } else if error_map.contains_key("description") {
                format!("Invalid description, \"{}\" is to short", note.content)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

    let result: Result<Note> = repository.create(user.0, note.0).await;

    match result {
        Ok(note) => Ok(HttpResponse::Ok().json(note)),
        Err(error) => {
            let pg_error: &PgError =
                error
                    .root_cause()
                    .downcast_ref::<PgError>()
                    .ok_or_else(|| {
                        debug!("Error creating note. {:?}", error);
                        AppError::INTERNAL_ERROR
                    })?;

            let error = match (pg_error.code(), pg_error.column_name()) {
                _ => {
                    debug!("Error creating note. {:?}", pg_error);
                    AppError::INTERNAL_ERROR.into()
                }
            };
            Err(error)
        }
    }
}

#[instrument[skip(repository)]]
pub async fn update_note(
    user: AuthenticatedUser,
    note: Json<UpdateNote>,
    id_note: String,
    repository: NoteRepository,
) -> AppResponse {
    match note.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("title") {
                format!("Invalid title, too short",)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

    let note = repository
        .update_note(user.0.to_string(), note.0, id_note.to_string())
        .await?;

    Ok(HttpResponse::Ok().json(note))
}

#[instrument[skip(repository)]]
pub async fn get_note(
    user: AuthenticatedUser,
    id_note: String,
    repository: NoteRepository,
) -> AppResponse {
    let note = repository
        .find_by_id(user.0, id_note)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(note))
}
