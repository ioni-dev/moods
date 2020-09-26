use actix_web::{web, App};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::Type, sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Attendees {
    pub id: String,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Appointment {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub meeting_partners: Option<Json<Vec<Attendees>>>,
    pub client_attendees: Option<Json<Vec<Attendees>>>,
    pub is_completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_user: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub id_project: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_contact: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewAppointment {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 10))]
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub meeting_partners: Option<Json<Vec<Attendees>>>,
    pub client_attendees: Option<Json<Vec<Attendees>>>,
    pub is_completed: bool,
    pub id_user: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub id_project: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_contact: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAppointment {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 10))]
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub meeting_partners: Option<Json<Vec<Attendees>>>,
    pub client_attendees: Option<Json<Vec<Attendees>>>,
    pub is_completed: bool,
    pub id_user: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub id_project: Option<Uuid>,
    pub id_lead: Option<Uuid>,
    pub id_contact: Option<Uuid>,
}
