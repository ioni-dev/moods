use actix_web::{web, App};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Json;
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::Type, sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Attendees {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Appointment {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub meeting_partners: Option<Json<Attendees>>,
    pub client_attendees: Option<Json<Attendees>>,
    pub is_completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_user: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub id_project: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_contact: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewAppointment {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 10))]
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub meeting_partners: Option<Json<Attendees>>,
    pub client_attendees: Option<Json<Attendees>>,
    pub is_completed: bool,
    pub id_user: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub id_project: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_contact: Option<Uuid>,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Validate)]
pub struct UpdateAppointment {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 10))]
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub meeting_partners: Option<Json<Attendees>>,
    pub client_attendees: Option<Json<Attendees>>,
    pub is_completed: bool,
    pub id_user: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub id_project: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_contact: Option<Uuid>,
}
