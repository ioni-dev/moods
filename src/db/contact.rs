use crate::{
    errors::AppError,
    models::contact::{Contact, NewContact, UpdateContact},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tracing::instrument;
use uuid::Uuid;
pub struct ContactRepository {
    pool: Arc<PgPool>,
}

impl ContactRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, new_contact))]
    pub async fn create(&self, new_contact: NewContact) -> Result<Contact> {
        let contact = sqlx::query_as::<_, Contact>(
            "insert into contacts (first_name, middle_name, last_name, email, cell_phone_number, linkedin, facebook,  twitter,
                website, position, logs, work_phone, is_active, last_talked_to, birthday, company_name, company_website, pic_url, last_consulted_at, 
                id_organization, id_note, id_user) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21) returning *",
        )
        .bind(new_contact.first_name)
        .bind(new_contact.middle_name)
        .bind(new_contact.last_name)
        .bind(new_contact.email)
        .bind(new_contact.cell_phone_number)
        .bind(new_contact.linkedin)
        .bind(new_contact.facebook)
        .bind(new_contact.twitter)
        .bind(new_contact.website)
        .bind(new_contact.position)
        .bind(new_contact.logs)
        .bind(new_contact.work_phone)
        .bind(new_contact.is_active)
        .bind(new_contact.last_talked_to)
        .bind(new_contact.birthday)
        .bind(new_contact.company_name)
        .bind(new_contact.company_website)
        .bind(new_contact.pic_url)
        .bind(new_contact.last_consulted_at)
        .bind(new_contact.id_organization)
        .bind(new_contact.id_note)
        .bind(new_contact.id_user)
        .fetch_one(&*self.pool)
        .await?;

        println!("{:?}", contact);
        Ok(contact)
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self, id_user: Uuid) -> Result<Vec<Contact>> {
        let mut all_contacts = vec![];

        let result = sqlx::query!(
            r#"
                SELECT *
                FROM contacts
                where id_user = $1"#,
            id_user
        )
        .fetch_all(&*self.pool)
        .await?;

        for contact in result {
            all_contacts.push(Contact {
                id: contact.id,
                first_name: contact.first_name,
                middle_name: contact.middle_name,
                last_name: contact.last_name,
                email: contact.email,
                cell_phone_number: contact.cell_phone_number,
                linkedin: contact.linkedin,
                facebook: contact.facebook,
                twitter: contact.twitter,
                website: contact.website,
                position: contact.position,
                logs: contact.logs,
                work_phone: contact.work_phone,
                is_active: contact.is_active,
                last_talked_to: contact.last_talked_to,
                birthday: contact.birthday,
                company_name: contact.company_name,
                company_website: contact.company_website,
                pic_url: contact.pic_url,
                last_consulted_at: contact.last_consulted_at,
                created_at: contact.created_at,
                updated_at: contact.updated_at,
                id_organization: contact.id_organization,
                id_note: contact.id_note,
                id_user: contact.id_user,
            })
        }

        Ok(all_contacts)
    }

    pub async fn update_contact(&self, id: String, contact: UpdateContact) -> Result<Contact> {
        let id = uuid::Uuid::parse_str(&id)?;
        let contact = sqlx::query_as::<_, Contact>(
            "update contacts set 
                first_name = $1,
                middle_name: = $2,
                last_name: = $3,
                email = $4
                cell_phone_number: = $5,
                linkedin: = $6,
                facebook: = $7,
                twitter: = $8,
                website: = $9,
                position = $10,
                logs: = $11,
                work_phone = $12,
                is_active: = $13,
                last_talked_to: = $14,
                birthday: = $15,
                company_name: = $16,
                company_website: = $17,
                pic_url: = $18,
                last_consulted_at: = $19,
                id_organization = $20,
                id_note = $21,
                WHERE id = $22 returning *",
        )
        .bind(contact.first_name)
        .bind(contact.middle_name)
        .bind(contact.last_name)
        .bind(contact.email)
        .bind(contact.cell_phone_number)
        .bind(contact.linkedin)
        .bind(contact.facebook)
        .bind(contact.twitter)
        .bind(contact.website)
        .bind(contact.position)
        .bind(contact.logs)
        .bind(contact.work_phone)
        .bind(contact.is_active)
        .bind(contact.last_talked_to)
        .bind(contact.birthday)
        .bind(contact.company_name)
        .bind(contact.company_website)
        .bind(contact.pic_url)
        .bind(contact.last_consulted_at)
        .bind(contact.id_organization)
        .bind(contact.id_note)
        .bind(id)
        .fetch_one(&*self.pool)
        .await?;
        Ok(contact)
    }
}

impl FromRequest for ContactRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();
    #[instrument(skip(req, payload))]
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<PgPool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(ContactRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
