use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Contact {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub phone: Option<String>,
    pub linkedin: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub description: String,
    pub is_active: bool,
    pub last_talked_to: Option<NaiveDateTime>,
    pub birthday: Option<NaiveDateTime>,
    pub company: String,
    pub company_website: Option<String>,
    pub avatar_url: Option<String>,
    pub last_consulted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub organization_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewContact {
    #[validate(length(min = 5))]
    pub first_name: String,
    pub middle_name: Option<String>,
    #[validate(length(min = 5))]
    pub last_name: String,
    pub phone: Option<String>,
    pub linkedin: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub description: String,
    pub is_active: bool,
    pub last_talked_to: Option<NaiveDateTime>,
    pub birthday: Option<NaiveDateTime>,
    pub company: String,
    pub company_website: Option<String>,
    pub avatar_url: Option<String>,
    pub last_consulted_at: Option<NaiveDateTime>,
    pub organization_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateContact {
    #[validate(length(min = 15))]
    pub first_name: String,
    #[validate(length(min = 4))]
    pub last_name: String,
    #[validate(length(min = 7))]
    pub phone: String,
    pub last_talked_to: NaiveDateTime,
    pub company: String,
    pub company_website: String,
    pub avatar_url: String,
    pub last_consulted_at: NaiveDateTime,
}
