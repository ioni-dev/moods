use crate::{
    config::crypto::CryptoService,
    errors::AppError,
    models::appointment::{Appointment, Attendees, NewAppointment, UpdateAppointment},
};
use actix_web::{web::Data, FromRequest};
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use color_eyre::Result;
use futures::future::{ready, Ready};
use serde_json::json;
use sqlx::postgres::PgQueryAs;
use sqlx::types::Json;
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
            "insert into appointments (name, description, start_date, end_date, meeting_partners, client_attendees, is_completed,
                id_user, id_note, id_project, id_lead, id_contact) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) returning *",
        )
        .bind(new_appointment.name)
        .bind(new_appointment.description)
        .bind(new_appointment.start_date)
        .bind(new_appointment.end_date)
        .bind(new_appointment.meeting_partners)
        .bind(new_appointment.client_attendees)
        .bind(new_appointment.is_completed)
        .bind(new_appointment.id_user)
        .bind(new_appointment.id_note)
        .bind(new_appointment.id_project)
        .bind(new_appointment.id_lead)
        .bind(new_appointment.id_contact)
        .fetch_one(&*self.pool)
        .await?;

        println!("{:?}", appointment);
        Ok(appointment)
    }

    pub async fn update_appointment(
        &self,
        id_user: String,
        appointment: UpdateAppointment,
        id_appointment: String,
    ) -> Result<Appointment> {
        let id_user = uuid::Uuid::parse_str(&id_user)?;
        let id_appointment = uuid::Uuid::parse_str(&id_appointment)?;

        let appointment = sqlx::query_as::<_, Appointment>(
            "update appointments set name = $1, description = $2, start_date = $3, end_date = $4, meeting_partners = $5, client_attendees = $6,
             is_completed = $7, id_user = $8, id_note = $9, id_project = $10, id_lead = $11, id_contact = $12 where id_user = $13 and id = $14 returning *",
        )
        .bind(appointment.name)
        .bind(appointment.description)
        .bind(appointment.start_date)
        .bind(appointment.end_date)
        .bind(json!(appointment.meeting_partners))
        .bind(json!(appointment.client_attendees))
        .bind(appointment.is_completed)
        .bind(appointment.id_user)
        .bind(appointment.id_note)
        .bind(appointment.id_project)
        .bind(appointment.id_lead)
        .bind(appointment.id_contact)
        .bind(id_user)
        .bind(id_appointment)
        .fetch_one(&*self.pool)
        .await?;
        Ok(appointment)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(
        &self,
        id_user: Uuid,
        id_appointment: String,
    ) -> Result<Option<Appointment>> {
        // let id_user = uuid::Uuid::parse_str(&id_user)?;
        let id_appointment = uuid::Uuid::parse_str(&id_appointment)?;
        let appointment = sqlx::query_as::<_, Appointment>(
            "select * from appointment where id = $2 and id_user = $1",
        )
        .bind(id_user)
        .bind(id_appointment)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(appointment)
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self, id_user: String) -> Result<Vec<Appointment>> {
        let id_user = uuid::Uuid::parse_str(&id_user)?;
        let mut all_appointments = vec![];

        let result = sqlx::query!(
            r#"
        SELECT *
        FROM appointments
        where id_user = $1"#,
            id_user
        )
        .fetch_all(&*self.pool)
        .await?;

        for appointment in result {
            let partners: Option<Json<Vec<Attendees>>> =
                serde_json::from_str(&appointment.meeting_partners.unwrap().to_string())?;

            let clients: Option<Json<Vec<Attendees>>> =
                serde_json::from_str(&appointment.client_attendees.unwrap().to_string())?;

            all_appointments.push(Appointment {
                id: appointment.id,
                name: appointment.name,
                description: appointment.description,
                start_date: appointment.start_date,
                end_date: appointment.end_date,
                meeting_partners: partners,
                client_attendees: clients,
                is_completed: appointment.is_completed,
                created_at: appointment.created_at,
                updated_at: appointment.updated_at,
                id_user: appointment.id_user,
                id_note: appointment.id_note,
                id_project: appointment.id_project,
                id_lead: appointment.id_lead,
                id_contact: appointment.id_contact,
            });
        }

        Ok(all_appointments)
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
