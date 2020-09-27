use crate::{
    errors::AppError,
    models::note::{Attachment, NewNote, Note, UpdateNote},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use serde_json::json;
use sqlx::postgres::PgQueryAs;
use sqlx::types::Json;
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
    pub async fn create(&self, id_user: Uuid, new_note: NewNote) -> Result<Note> {
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
            let attachments: Option<Json<Vec<Attachment>>> =
                serde_json::from_str(&note.attachment_path.unwrap().to_string())?;

            all_notes.push(Note {
                id: note.id,
                title: note.title,
                content: note.content,
                attachment_path: attachments,
                id_contact: note.id_contact,
                id_lead: note.id_lead,
                id_task: note.id_task,
                id_bid: note.id_bid,
                id_appointment: note.id_appointment,
                id_project: note.id_project,
                id_user: note.id_user,
                created_at: note.created_at,
                updated_at: note.updated_at,
            });
        }

        Ok(all_notes)
    }

    pub async fn update_note(
        &self,
        id_user: String,
        note: UpdateNote,
        id_note: String,
    ) -> Result<Note> {
        let id_user = uuid::Uuid::parse_str(&id_user)?;
        let id_note = uuid::Uuid::parse_str(&id_note)?;

        let note = sqlx::query_as::<_, Note>(
            "update notes set title = $1, content = $2, attachment_path = $3, id_contact = $4, id_lead = $5, id_task = $6,
             id_bid = $7, id_appointment = $8, id_project = $9 where id_user = $10 and id = $11 returning *",
        )
        .bind(note.title)
        .bind(note.content)
        .bind(json!(note.attachment_path))
        .bind(note.id_contact)
        .bind(note.id_lead)
        .bind(note.id_task)
        .bind(note.id_bid)
        .bind(note.id_appointment)
        .bind(note.id_project)
        .bind(id_user)
        .bind(id_note)
        .fetch_one(&*self.pool)
        .await?;
        Ok(note)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id_user: Uuid, id_note: String) -> Result<Option<Note>> {
        // let id_user = uuid::Uuid::parse_str(&id_user)?;
        let id_note = uuid::Uuid::parse_str(&id_note)?;
        let note = sqlx::query_as::<_, Note>("select * from notes where id = $2 and id_user = $1")
            .bind(id_user)
            .bind(id_note)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(note)
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
