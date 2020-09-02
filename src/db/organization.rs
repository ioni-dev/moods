use crate::{
    config::crypto::CryptoService,
    errors::AppError,
    models::organization::{NewOrganization, Organization, UpdateOrganization},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tracing::instrument;
use uuid::Uuid;

pub struct OrganizationRepository {
    pool: Arc<PgPool>,
}

impl OrganizationRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self) -> Result<Option<Organization>> {
        let all_organizations = sqlx::query_as::<_, Organization>("select * from organizations")
            .fetch_optional(&*self.pool)
            .await?;

        Ok(all_organizations)
    }

    #[instrument(skip(self, new_organization))]
    pub async fn create(&self, new_organization: NewOrganization) -> Result<Organization> {
        let organization = sqlx::query_as::<_, Organization>(
            "insert into organizations (name, address, website, email, password_hash, active,  email_verified,
                max_employees, max_users, phone) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning *",
        )
        .bind(new_organization.name)
        .bind(new_organization.address)
        .bind(new_organization.website)
        .bind(new_organization.email)
        .bind(new_organization.password_hash)
        .bind(new_organization.active)
        .bind(new_organization.email_verified)
        .bind(new_organization.max_employees)
        .bind(new_organization.max_users)
        .bind(new_organization.phone)
        .fetch_one(&*self.pool)
        .await?;

        // println!("{:?}", contact);
        Ok(organization)
    }
}

impl FromRequest for OrganizationRepository {
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
            Ok(pool) => ready(Ok(OrganizationRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
