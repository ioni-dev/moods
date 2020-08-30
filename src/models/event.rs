use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::Type)]
pub enum EventCategory {
    Wedding,
    Travel,
    Party,
    Conference,
    Other,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Event {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: String,
    pub category: EventCategory,
    pub location: Vec<f64>,
    pub date: String,
    pub rules: Vec<String>,
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
    pub category_tag: String,
    pub active: bool,

}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEvent {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 3))]
    pub description: String,
    #[serde(skip_serializing)]
    pub category: EventCategory,
    #[validate(length(min = 1))]
    pub date: String,
    pub location: Vec<f64>,
}
