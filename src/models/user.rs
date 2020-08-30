use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing)]
    pub email_verified: bool,
    #[serde(skip_serializing)]
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfile {
    pub name: Option<String>,
    pub password_hash: Option<String>,
}
