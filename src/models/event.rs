use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub enum event_category {
    wedding,
    travel,
    party,
    conference,
    other,
}
#[derive(sqlx::FromRow, Serialize)]
pub struct Event {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: String,
    pub category: event_category,
    pub location: Option<String>,
    pub date: String,
    pub rules: String,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewEvent {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 3))]
    pub description: String,
    #[validate(length(min = 1))]
    pub category: event_category,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEvent {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 3))]
    pub description: String,
    #[validate(length(min = 1))]
    pub category: event_category,
    #[validate(length(min = 1))]
    pub date: String,
    #[validate(length(min = 1))]
    pub location: Option<String>,
}
