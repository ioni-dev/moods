use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub cell_phone_number: Option<String>,
    pub linkedin: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub position: Option<String>,
    pub logs: Option<String>,
    pub work_phone: Option<String>,
    pub is_active: bool,
    pub last_talked_to: Option<NaiveDateTime>,
    pub birthday: Option<NaiveDate>,
    pub company_name: Option<String>,
    pub company_website: Option<String>,
    pub pic_url: Option<String>,
    pub last_consulted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_organization: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub id_user: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewContact {
    #[validate(length(min = 5))]
    pub first_name: String,
    pub middle_name: Option<String>,
    #[validate(length(min = 5))]
    pub last_name: String,
    pub email: String,
    pub cell_phone_number: Option<String>,
    pub linkedin: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub position: Option<String>,
    pub logs: Option<String>,
    pub work_phone: Option<String>,
    pub is_active: bool,
    pub last_talked_to: Option<NaiveDateTime>,
    pub birthday: Option<NaiveDate>,
    pub company_name: Option<String>,
    pub company_website: Option<String>,
    pub pic_url: Option<String>,
    pub last_consulted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_organization: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub id_user: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateContact {
    #[validate(length(min = 5))]
    pub first_name: String,
    pub middle_name: Option<String>,
    #[validate(length(min = 5))]
    pub last_name: String,
    pub email: String,
    pub cell_phone_number: Option<String>,
    pub linkedin: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub position: Option<String>,
    pub logs: Option<String>,
    pub work_phone: Option<String>,
    pub is_active: bool,
    pub last_talked_to: Option<NaiveDateTime>,
    pub birthday: Option<NaiveDate>,
    pub company_name: Option<String>,
    pub company_website: Option<String>,
    pub pic_url: Option<String>,
    pub last_consulted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id_organization: Option<Uuid>,
    pub id_note: Option<Uuid>,
    pub id_user: Option<Uuid>,
}
