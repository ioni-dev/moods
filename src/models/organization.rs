use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub website: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing)]
    pub email_verified: bool,
    pub max_employees: i32,
    pub max_users: i32,
    pub phone: String,
    #[serde(skip_serializing)]
    pub active: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewOrganization {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(length(min = 4))]
    pub address: String,
    pub website: String,
    #[validate(email)]
    pub email: String,
    pub phone: String,
    pub password: String,
    pub max_employees: i32,
    pub max_users: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrganization {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(length(min = 4))]
    pub address: String,
    pub website: String,
    pub email: String,
    pub max_employees: i32,
    pub max_users: i32,
    pub phone: String,
}
