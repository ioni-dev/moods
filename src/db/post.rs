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

pub struct PostRepository {
    pool: Arc<PgPool>,
}

impl PostRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, new_post))]
    pub async fn create(&self, new_post: NewPost, event: Event) -> Result<Post> {
        let post = sqlx::query_as::<_, Post>(
            "with new_post as (
            insert into posts (title, content, show_comments) values ($1, $2, $3) returning id
            )
            insert into event_posts (event_id, post_id)
            values ( event.id, (select id from new_post)) returning *",
        )
        .bind(new_post.title)
        .bind(new_post.content)
        .bind(new_post.show_comments)
        .fetch_one(&*self.pool)
        .await?;
        Ok(post)
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self, id: Uuid) -> Result<Option<Post>> {
        let all_post = sqlx::query_as::<_, Event>("select * from posts where id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(all_post)
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
