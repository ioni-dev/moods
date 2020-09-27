use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::Type, sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Attachment {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub attachment_path: Option<Json<Vec<Attachment>>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_contact: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_task: Option<Uuid>,
    pub id_bid: Option<Uuid>,
    pub id_appointment: Option<Uuid>,
    pub id_project: Option<Uuid>,
    pub id_user: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewNote {
    #[validate(length(min = 15))]
    pub title: String,
    #[validate(length(min = 50, max = 1200))]
    pub content: String,
    pub attachment_path: Option<Json<Vec<Attachment>>>,
    pub id_contact: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_task: Option<Uuid>,
    pub id_bid: Option<Uuid>,
    pub id_appointment: Option<Uuid>,
    pub id_project: Option<Uuid>,
    pub id_user: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateNote {
    #[validate(length(min = 15))]
    pub title: String,
    #[validate(length(min = 50, max = 1200))]
    pub content: String,
    pub attachment_path: Option<Json<Vec<Attachment>>>,
    pub id_contact: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_task: Option<Uuid>,
    pub id_bid: Option<Uuid>,
    pub id_appointment: Option<Uuid>,
    pub id_project: Option<Uuid>,
    pub id_user: Option<Uuid>,
}
