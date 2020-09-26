use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    config::crypto::CryptoService,
    db,
    db::note::NoteRepository,
    errors::AppError,
    models::note::{Note, NewNote, UpdateNote},
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use color_eyre::Result;
// use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, instrument};

#[instrument[skip(repository)]]
pub async fn get_all_posts(user: AuthenticatedUser, repository: PostRepository) -> AppResponse {
    let posts = repository
        .get_all(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(posts))
}

pub async fn create_post(
    post: Json<NewPost>,
    repository: PostRepository,
    event: Event
) -> AppResponse {
    match post.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("title") {
                format!("Invalid title. \"{}\" is too short.", post.title)
            } else if error_map.contains_key("content") {
                format!("Invalid content \"{}\" is to short", post.content)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;
