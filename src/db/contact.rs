use crate::{
    config::crypto::CryptoService,
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
            "insert into contacts (first_name, middle_name, last_name, phone, linkedin, facebook,  twitter,
                website, description, is_active, last_talked_to, birthday, company, company_website, avatar_url, last_consulted_at, organization_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) returning *",
        )
        .bind(new_contact.first_name)
        .bind(new_contact.middle_name)
        .bind(new_contact.last_name)
        .bind(new_contact.phone)
        .bind(new_contact.linkedin)
        .bind(new_contact.facebook)
        .bind(new_contact.twitter)
        .bind(new_contact.website)
        .bind(new_contact.description)
        .bind(new_contact.is_active)
        .bind(new_contact.last_talked_to)
        .bind(new_contact.birthday)
        .bind(new_contact.company)
        .bind(new_contact.company_website)
        .bind(new_contact.avatar_url)
        .bind(new_contact.last_consulted_at)
        .bind(new_contact.organization_id)
        .fetch_one(&*self.pool)
        .await?;

        println!("{:?}", contact);
        Ok(contact)
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self, id: Uuid) -> Result<Option<Contact>> {
        let all_contacts =
            sqlx::query_as::<_, Contact>("select * from contacts where user_id = $1")
                .bind(id)
                .fetch_optional(&*self.pool)
                .await?;

        Ok(all_contacts)
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
