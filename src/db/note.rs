use crate::{
    config::crypto::CryptoService,
    errors::AppError,
    models::note::{NewNote, Note, UpdateNote},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tracing::instrument;
use uuid::Uuid;

pub struct NoteRepository {
    pool: Arc<PgPool>,
}

impl NoteRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, new_note))]
    pub async fn create(&self, new_note: NewNote, note: Note) -> Result<Note> {
        let note = sqlx::query_as::<_, Note>(
            "insert into notes (title, content, attachment_path, id_contact, id_lead, id_task, id_bid, id_appointment, id_project) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *",
        )
        .bind(new_note.title)
        .bind(new_note.content)
        .bind(new_note.attachment_path)
        .bind(new_note.id_contact)
        .bind(new_note.id_lead)
        .bind(new_note.id_task)
        .bind(new_note.id_bid)
        .bind(new_note.id_appointment)
        .bind(new_note.id_project)
        .fetch_one(&*self.pool)
        .await?;
        Ok(note)
    }

    // TODO:
    //     - Add a more complex query to use the bridge table to bring  event table posts to the current user

    #[instrument(skip(self))]
    pub async fn get_all(&self, id_user: String) -> Result<Vec<Note>> {
        let id_user = uuid::Uuid::parse_str(&id_user)?;
        let mut all_notes = vec![];
        let result = sqlx::query!(
            r#"
            SELECT *
            FROM notes
            where id_user = $1"#,
            id_user
        )
        .fetch_all(&*self.pool)
        .await?;

        for note in result {
            all_notes.push(Note {
                id: note.id,
                title: note.title,
                content: note.content,
                attachment_path: note.attachment_path,
                id_contact: note.id_contact,
                id_lead: note.id_lead,
                id_task: note.id_task,
                id_bid: note.id_bid,
                id_appointment: note.id_appointment,
                id_project: note.id_project,
                created_at: note.created_at,
                updated_at: note.updated_at,
            });
        }

        Ok(all_notes)
    }
}

impl FromRequest for NoteRepository {
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
            Ok(pool) => ready(Ok(NoteRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
