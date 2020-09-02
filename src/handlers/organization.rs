use super::{auth::AuthenticatedUser, AppResponse};
use crate::{
    config::crypto::CryptoService,
    db,
    db::organization::OrganizationRepository,
    errors::AppError,
    models::organization::{NewOrganization, Organization, UpdateOrganization},
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use color_eyre::Result;
use sqlx::{error::DatabaseError, postgres::PgError};
use tracing::{debug, info, instrument};
use validator::Validate;

#[instrument[skip(repository)]]
pub async fn get_all_organizations(repository: OrganizationRepository) -> AppResponse {
    let organizations = repository
        .get_all()
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(organizations))
}
