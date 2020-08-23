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
    pub async fn get_all(&self, id: Uuid) -> Result<Option<Event>> {
        let all_events = sqlx::query_as::<_, Event>("select * from user_events where id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(all_events)
    }

}
