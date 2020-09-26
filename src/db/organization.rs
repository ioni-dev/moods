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
            "insert into organizations (name, address, website, email, password_hash, max_employees, max_users, phone) values ($1, $2, $3, $4, $5,$6, $7, $8) returning *",
        )
        .bind(new_organization.name)
        .bind(new_organization.address)
        .bind(new_organization.website)
        .bind(new_organization.email)
        .bind(new_organization.business_type)
        .bind(new_organization.tag)
        .bind(new_organization.active)
        .bind(new_organization.phone)
        .bind(new_organization.id_user)
        .bind(new_organization.id_contact)
        .fetch_one(&*self.pool)
        .await?;

        println!("{:?}", organization);
        Ok(organization)
    }

    pub async fn update_organization(
        &self,
        id_user: String,
        organization: UpdateOrganization,
        id_organization: String,
    ) -> Result<Organization> {
        let id_user = uuid::Uuid::parse_str(&id_user)?;
        let id_organization = uuid::Uuid::parse_str(&id_organization)?;

        let organization = sqlx::query_as::<_, Organization>(
            "update appointments set name = $1, address = $2, website = $3, email = $4, business_type = $5, tag = $6,
             active = $7, phone = $8, id_user = $9, id_contact = $10 where id_user = $13 and id = $14 returning *",
        )
        .bind(organization.name)
        .bind(organization.address)
        .bind(organization.website)
        .bind(organization.email)
        .bind(organization.business_type)
        .bind(organization.tag)
        .bind(organization.active)
        .bind(organization.phone)
        .bind(organization.id_user)
        .bind(organization.id_contact)
        .bind(id_user)
        .bind(id_organization)
        .fetch_one(&*self.pool)
        .await?;
        Ok(organization)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(
        &self,
        id_user: Uuid,
        id_organization: String,
    ) -> Result<Option<Organization>> {
        // let id_user = uuid::Uuid::parse_str(&id_user)?;
        let id_organization = uuid::Uuid::parse_str(&id_organization)?;
        let organization = sqlx::query_as::<_, Organization>(
            "select * from organizations where id = $2 and id_user = $1",
        )
        .bind(id_user)
        .bind(id_organization)
        .fetch_optional(&*self.pool)
        .await?;

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
