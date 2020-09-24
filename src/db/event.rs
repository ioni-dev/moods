use crate::{
    config::crypto::CryptoService,
    errors::AppError,
    models::event::{Event, NewEvent, UpdateEvent},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tracing::instrument;
use uuid::Uuid;

pub struct EventRepository {
    pool: Arc<PgPool>,
}

impl EventRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self, id_user: String) -> Result<Option<Event>> {
        let id_user = uuid::Uuid::parse_str(&id_user)?;
        let all_events = sqlx::query_as::<_, Event>("select * from event where id_user = $1")
            .bind(id_user)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(all_events)
    }
}

impl FromRequest for EventRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();
    #[instrument(skip(req, payload))]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<PgPool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(EventRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
