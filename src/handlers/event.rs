use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    db,
    config::crypto::CryptoService,
    db::event::EventRepository,
    errors::AppError,
    models::event::{Event, NewEvent, UpdateEvent},
};

#[instrument[skip(repository)]]
pub async fn get_all_events(user: AuthenticatedUser, repository: EventRepository) -> AppResponse {
    let events = repository
        .get_all(user.0)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(events))
}
