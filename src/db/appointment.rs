use crate::{
    config::crypto::CryptoService,
    errors::AppError,
    models::appointment::{Appointment, NewAppointment, UpdateAppointment},
};
use actix_web::{web::Data, FromRequest};
use color_eyre::Result;
use futures::future::{ready, Ready};
use serde_json::json;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tracing::instrument;
use uuid::Uuid;

pub struct AppointmentRepository {
    pool: Arc<PgPool>,
}

impl AppointmentRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, new_appointment))]
    pub async fn create(&self, new_appointment: NewAppointment) -> Result<Appointment> {
        let appointment = sqlx::query_as::<_, Appointment>(
            "insert into appointments (name, description, start_date, end_date, notes, meeting_partners,  client_attendees, is_completed,
                user_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *",
        )
        .bind(new_appointment.name)
        .bind(new_appointment.description)
        .bind(new_appointment.start_date)
        .bind(new_appointment.end_date)
        .bind(new_appointment.notes)
        .bind(json!(new_appointment.meeting_partners))
        .bind(json!(new_appointment.client_attendees))
        .bind(new_appointment.is_completed)
        .bind(new_appointment.user_id)
        .fetch_one(&*self.pool)
        .await?;

        println!("{:?}", appointment);
        Ok(appointment)
    }

    pub async fn update_appointment(
        &self,
        user_id: Uuid,
        appointment: UpdateAppointment,
        hashing: &CryptoService,
    ) -> Result<Appointment> {
        let user = sqlx::query_as::<_, Appointment>(
            "update appointments set name = $2, description = $3, start_date = $4, end_date = $5,
            notes = $6, meeting_partners = $7, client_attendees = $8 where id = $1 returning *",
        )
        .bind(user_id)
        .bind(appointment.name)
        .bind(appointment.description)
        .bind(appointment.start_date)
        .bind(appointment.end_date)
        .bind(appointment.notes)
        .bind(json!(appointment.meeting_partners))
        .bind(appointment.client_attendees)
        .fetch_one(&*self.pool)
        .await?;
        Ok(user)
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self, id: Uuid) -> Result<Option<Appointment>> {
        let all_appointments =
            sqlx::query_as::<_, Appointment>("select * from appointments where user_id = $1")
                .bind(id)
                .fetch_optional(&*self.pool)
                .await?;

        Ok(all_appointments)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Appointment>> {
        let maybe_appointment =
            sqlx::query_as::<_, Appointment>("select * from appointment where id = $1")
                .bind(id)
                .fetch_optional(&*self.pool)
                .await?;

        Ok(maybe_appointment)
    }
}
impl FromRequest for AppointmentRepository {
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
            Ok(pool) => ready(Ok(AppointmentRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
