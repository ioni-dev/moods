use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub attachment_path: Option<Json<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_contact: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_task: Option<Uuid>,
    pub id_bid: Option<Uuid>,
    pub id_appointment: Option<Uuid>,
    pub id_project: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewNote {
    #[validate(length(min = 15))]
    pub title: String,
    #[validate(length(min = 50, max = 1200))]
    pub content: String,
    pub attachment_path: Option<Json<String>>,
    pub id_contact: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_task: Option<Uuid>,
    pub id_bid: Option<Uuid>,
    pub id_appointment: Option<Uuid>,
    pub id_project: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNote {
    #[validate(length(min = 15))]
    pub title: String,
    #[validate(length(min = 50, max = 1200))]
    pub content: String,
    pub attachment_path: Option<Json<String>>,
    pub id_contact: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_task: Option<Uuid>,
    pub id_bid: Option<Uuid>,
    pub id_appointment: Option<Uuid>,
    pub id_project: Option<Uuid>,
}
