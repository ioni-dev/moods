use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_organization: Option<Uuid>,
    pub id_user: Option<Uuid>,
    pub position: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewEmployee {
    #[validate(length(min = 5))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub email_verified: bool,
    pub active: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_organization: Option<Uuid>,
    pub id_user: Option<Uuid>,
    pub position: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmployee {
    #[validate(length(min = 5))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub email_verified: bool,
    pub active: Option<bool>,
    pub id_organization: Option<Uuid>,
    pub id_user: Option<Uuid>,
    pub position: Option<String>,
}
