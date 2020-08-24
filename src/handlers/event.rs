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

#[instrument[skip(repository)]]
pub async fn get_all_events(user: AuthenticatedUser, repository: EventRepository) -> AppResponse {
    let events = repository
        .get_all(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(events))
}
