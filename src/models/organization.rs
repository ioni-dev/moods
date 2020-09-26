use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, sqlx::FromRow, Serialize, Validate)]
pub struct Organization {
    pub id: Uuid,
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(length(min = 4))]
    pub address: String,
    pub website: Option<String>,
    pub email: Option<String>,
    pub business_type: Option<String>,
    pub tag: Option<String>,
    pub active: bool,
    pub phone: Option<String>,
    pub id_user: Option<Uuid>,
    pub id_contact: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewOrganization {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(length(min = 4))]
    pub address: String,
    pub website: Option<String>,
    pub email: Option<String>,
    pub business_type: Option<String>,
    pub tag: Option<String>,
    pub active: bool,
    pub phone: Option<String>,
    pub id_user: Option<Uuid>,
    pub id_contact: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrganization {
    #[validate(length(min = 4))]
    pub name: String,
    #[validate(length(min = 4))]
    pub address: String,
    pub website: Option<String>,
    pub email: Option<String>,
    pub business_type: Option<String>,
    pub tag: Option<String>,
    pub active: bool,
    pub phone: Option<String>,
    pub id_user: Option<Uuid>,
    pub id_contact: Option<Uuid>,
}
