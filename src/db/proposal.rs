use crate::{
    errors::AppError,
    models::proposal::{NewProposal, Proposal, StatusTag, UpdateProposal},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use serde_json::json;
use sqlx::postgres::PgQueryAs;
use sqlx::types::Json;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tracing::instrument;
use uuid::Uuid;

pub struct ProposalRepository {
    pool: Arc<PgPool>,
}

impl ProposalRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, new_proposal))]
    pub async fn create(&self, id_user: Uuid, new_proposal: NewProposal) -> Result<Proposal> {
        let employee = sqlx::query_as::<_, Proposal>(
            "insert into proposals (name, status, introduction, attachment_path, id_contact, id_user) values ($1, $2, $3, $4, $5, $6) returning *",
        )
        .bind(new_proposal.name)
        .bind(new_proposal.status)
        .bind(new_proposal.introduction)
        .bind(new_proposal.attachment_path)
        .bind(new_proposal.estimate)
        .bind(new_proposal.id_contact)
        .bind(id_user)
        .fetch_one(&*self.pool)
        .await?;
        Ok(employee)
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self, id_user: Uuid) -> Result<Vec<Proposal>> {
        // let id_user = uuid::Uuid::parse_str(&id_user)?;
        let mut all_proposals = vec![];
        // let result = sqlx::query!(
        //     r#"
        //     SELECT *
        //     FROM proposals
        //     where id_user = $1"#,
        //     id_user
        // )
        // .fetch_all(&*self.pool)
        // .await?;

        for proposal in result {
            let attachment_path: Option<Json<String>> =
                serde_json::from_str(&proposal.attachment_path.unwrap().to_string())?;

            let estimate =
                serde_json::from_str::<StatusTag>(&proposal.estimate.to_string()).unwrap();

            all_proposals.push(Proposal {
                id: proposal.id,
                name: proposal.name,
                status: proposal.status,
                introduction: proposal.introduction,
                attachment_path: attachment_path,
                estimate: estimate,
                created_at: proposal.created_at,
                updated_at: proposal.updated_at,
                id_contact: proposal.id_contact,
                id_user: proposal.id_user,
            });
        }

        Ok(all_proposals)
    }

    pub async fn update_proposal(
        &self,
        id_user: String,
        proposal: UpdateProposal,
        id_proposal: String,
    ) -> Result<Proposal> {
        let id_user = uuid::Uuid::parse_str(&id_user)?;
        let id_proposal = uuid::Uuid::parse_str(&id_proposal)?;

        let proposal = sqlx::query_as::<_, Proposal>(
            "update proposal set name = $1, status = $2, introduction = $3, attachment_path = $4, estimate = $5, id_contac = $6,
             where id_user = $7 and id = $8 returning *",
        )
        .bind(proposal.name)
        .bind(proposal.status)
        .bind(proposal.introduction)
        .bind(proposal.attachment_path)
        .bind(proposal.estimate)
        .bind(proposal.id_contact)
        .bind(proposal.id_user)
        .fetch_one(&*self.pool)
        .await?;
        Ok(proposal)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id_user: Uuid, id_proposal: String) -> Result<Option<Proposal>> {
        // let id_user = uuid::Uuid::parse_str(&id_user)?;
        let id_proposal = uuid::Uuid::parse_str(&id_proposal)?;
        let proposal =
            sqlx::query_as::<_, Proposal>("select * from proposal where id = $2 and id_user = $1")
                .bind(id_user)
                .bind(id_proposal)
                .fetch_optional(&*self.pool)
                .await?;

        Ok(proposal)
    }
}

impl FromRequest for ProposalRepository {
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
            Ok(pool) => ready(Ok(ProposalRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
