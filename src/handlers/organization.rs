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

#[instrument(skip(organization, repository, crypto_service))]
pub async fn create_organization(
    organization: Json<NewOrganization>,
    repository: OrganizationRepository,
    crypto_service: Data<CryptoService>,
) -> AppResponse {
    match organization.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("name") {
                format!("Invalid name. \"{}\" is too short.", organization.name)
            } else if error_map.contains_key("address") {
                format!("Invalid address \"{:?}\"", organization.email)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

    let result: Result<Organization> = repository.create(organization.0).await;

    match result {
        Ok(organization) => Ok(HttpResponse::Ok().json(organization)),
        Err(error) => {
            let pg_error: &PgError =
                error
                    .root_cause()
                    .downcast_ref::<PgError>()
                    .ok_or_else(|| {
                        debug!("Error creating organization. {:?}", error);
                        AppError::INTERNAL_ERROR
                    })?;

            let error = match (pg_error.code(), pg_error.column_name()) {
                (Some(db::UNIQUE_VIOLATION_CODE), Some("name")) => {
                    AppError::INVALID_INPUT.message("name already exists.".to_string())
                }
                (Some(db::UNIQUE_VIOLATION_CODE), None) => {
                    AppError::INVALID_INPUT.message("name  already exists.".to_string())
                }
                _ => {
                    debug!("Error creating organization. {:?}", pg_error);
                    AppError::INTERNAL_ERROR.into()
                }
            };
            Err(error)
        }
    }
}

#[instrument[skip(repository)]]
pub async fn update_organization(
    user: AuthenticatedUser,
    appointment: Json<UpdateOrganization>,
    id_organization: String,
    repository: OrganizationRepository,
) -> AppResponse {
    match appointment.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_map = errors.field_errors();

            let message = if error_map.contains_key("name") {
                format!("Invalid name, too short",)
            } else {
                "Invalid input.".to_string()
            };

            Err(AppError::INVALID_INPUT.message(message))
        }
    }?;

    let organization = repository
        .update_organization(
            user.0.to_string(),
            appointment.0,
            id_organization.to_string(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(organization))
}

#[instrument[skip(repository)]]
pub async fn get_all_organizations(repository: OrganizationRepository) -> AppResponse {
    let organizations = repository
        .get_all()
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(organizations))
}

#[instrument[skip(repository)]]
pub async fn get_organization(
    user: AuthenticatedUser,
    id_organization: String,
    repository: OrganizationRepository,
) -> AppResponse {
    let organization = repository
        .find_by_id(user.0, id_organization)
        .await?
        .ok_or(AppError::INTERNAL_ERROR)?;

    Ok(HttpResponse::Ok().json(organization))
}
