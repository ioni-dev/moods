use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(rename = "meeting_partners")]
pub struct MeetingPartners {
    id: String,
    name: String,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(rename = "client_attendees")]
pub struct ClientAttendees {
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
    // pub meeting_partners: Vec<MeetingPartners>,
    // pub client_attendees: Vec<ClientAttendees>,
    pub is_completed: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewAppointment {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(length(min = 15))]
    pub description: String,
    // #[validate(Required)]
    pub start_date: NaiveDateTime,
    // #[validate(Required)]
    pub end_date: NaiveDateTime,
    pub notes: String,
    // #[serde(skip_serializing)]
    // pub meeting_partners: Vec<MeetingPartners>,
    // #[serde(skip_serializing)]
    // pub client_attendees: Vec<ClientAttendees>,
    // #[validate(Required)]
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAppointment {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(length(min = 15))]
    pub description: String,
    // #[validate(Required)]
    pub start_date: NaiveDateTime,
    // #[validate(Required)]
    pub end_date: NaiveDateTime,
    pub notes: String,
    // pub meeting_partners: Vec<MeetingPartners>,
    // pub client_attendees: Vec<ClientAttendees>,
}
