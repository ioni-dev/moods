use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize)]
pub struct Contact {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub phone: String,
    pub linkedin: String,
    pub facebook: String,
    pub twitter: String,
    pub website: String,
    pub description: String,
    pub is_active: bool,
    pub last_talked_to: NaiveDateTime,
    pub birthday: NaiveDateTime,
    pub company: String,
    pub company_website: String,
    pub avatar_url: String,
    pub last_consulted_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub organization_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewContact {
    #[validate(length(min = 15))]
    pub first_name: String,
    pub middle_name: String,
    #[valdate(Required)]
    pub last_name: String,
    #[valdate(Required)]
    pub phone: String,
    pub linkedin: String,
    pub facebook: String,
    pub twitter: String,
    pub website: String,
    #[valdate(Required)]
    pub description: String,
    #[valdate(Required)]
    pub is_active: bool,
    pub last_talked_to: NaiveDateTime,
    pub birthday: NaiveDateTime,
    #[valdate(Required)]
    pub company: String,
    pub company_website: String,
    pub avatar_url: String,
    pub last_consulted_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateContact {
    #[validate(length(min = 15))]
    pub first_name: String,
    #[valdate(Required)]
    pub last_name: String,
    #[valdate(Required)]
    pub phone: String,
    pub last_talked_to: NaiveDateTime,
    pub company: String,
    pub company_website: String,
    pub avatar_url: String,
    pub last_consulted_at: NaiveDateTime,
}
