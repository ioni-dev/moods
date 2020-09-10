use actix_web::{web, App};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Json;
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::Type, sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Attendees {
    id: String,
    name: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Appointment {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub notes: String,
    pub meeting_partners: Json<Attendees>,
    pub client_attendees: Json<Attendees>,
    pub is_completed: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewAppointment {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 10))]
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub notes: Option<String>,
    pub meeting_partners: Json<Attendees>,
    pub client_attendees: Json<Attendees>,
    pub is_completed: bool,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAppointment {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(length(min = 15))]
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub notes: Option<String>,
    pub meeting_partners: Option<Json<Attendees>>,
    pub client_attendees: Option<Json<Attendees>>,
}
