use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug)]
pub enum StatusTag {
    Approved,
    #[serde(rename = "On hold")]
    OnHold,
    Declined,
    Closed,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Proposal {
    pub id: Uuid,
    pub name: String,
    pub status: Option<String>,
    pub introduction: String,
    pub attachment_path: Option<Json<String>>,
    pub estimate: StatusTag,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_contact: Option<Uuid>,
    pub id_user: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewProposal {
    #[validate(length(min = 3))]
    pub name: String,
    pub status: Option<String>,
    #[validate(length(min = 15))]
    pub introduction: String,
    pub attachment_path: Option<Json<String>>,
    pub estimate: StatusTag,
    pub id_contact: Option<Uuid>,
    pub id_user: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProposal {
    #[validate(length(min = 3))]
    pub name: String,
    pub status: Option<String>,
    #[validate(length(min = 15))]
    pub introduction: String,
    pub attachment_path: Option<Json<String>>,
    pub estimate: StatusTag,
    pub id_contact: Option<Uuid>,
    pub id_user: Option<Uuid>,
}
