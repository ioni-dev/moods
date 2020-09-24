use actix_web::{web, App};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::types::Json;
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::Type, sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Attendees {
    pub id: String,
    pub name: String,
}

// mod opt_external_struct {
//     use super::{ExternalStructDef, Attendees};
//     use serde::{Serialize, Serializer, Deserialize, Deserializer};

//     pub fn serialize<S>(value: &Option<Attendees>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         #[derive(Serialize)]
//         struct Helper<'a>(#[serde(with = "ExternalStructDef")] &'a Attendees);

//         value.as_ref().map(Helper).serialize(serializer)
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Attendees>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         struct Helper(#[serde(with = "ExternalStructDef")] Attendees);

//         let helper = Option::deserialize(deserializer)?;
//         Ok(helper.map(|Helper(external)| external))
//     }
// }

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
