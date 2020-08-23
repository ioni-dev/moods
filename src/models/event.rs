use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub enum EventCategory {
    Wedding,
    Travel,
    Party,
    Conference,
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    x: f64,
    y: f64
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Event {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: String,
    pub category: EventCategory,
    pub location: Box<Point>,
    pub date: String,
    pub rules: Box<[i32]>,
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
    pub category: EventCategory,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEvent {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 3))]
    pub description: String,
    pub category: EventCategory,
    #[validate(length(min = 1))]
    pub date: String,
    pub location: Box<Point>
}
