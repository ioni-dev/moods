mod appointment;
mod auth;
mod contact;
mod event;
mod organization;
mod user;

use crate::errors::AppError;
use actix_web::{web, HttpResponse};
use appointment::{create_appointment, get_all_appointments, update_appointment};
use auth::auth;
use contact::{create_contact, get_all_contacts};
use event::get_all_events;
use organization::{create_organization, get_all_organizations};
use user::{create_user, profile, update_profile};
type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut web::ServiceConfig) {
    let organization_signup =
        web::resource("/organization-signup").route(web::post().to(create_organization));

    let all_organizations =
        web::resource("/all-organizations").route(web::get().to(get_all_organizations));

    let signup = web::resource("/signup").route(web::post().to(create_user));

    let auth = web::resource("/auth").route(web::post().to(auth));

    let profile = web::resource("/profile")
        .route(web::get().to(profile))
        .route(web::post().to(update_profile));

    // events related to user
    // let create_event: web::resource("/new_event").route(web::get().to(create_event));
    let all_events = web::resource("/events").route(web::get().to(get_all_events));

    let all_contacts = web::resource("/contacts").route(web::get().to(get_all_contacts));
    let create_contact = web::resource("/new-contacts").route(web::post().to(create_contact));

    let create_appointment =
        web::resource("/create-appointment").route(web::post().to(create_appointment));

    let modify_appointment =
        web::resource("/update-appointment").route(web::post().to(update_appointment));

    let all_appointments =
        web::resource("/all-appointments").route(web::get().to(get_all_appointments));

    config
        .service(signup)
        .service(auth)
        .service(profile)
        .service(all_events)
        .service(all_contacts)
        .service(create_contact)
        .service(organization_signup)
        .service(all_organizations)
        .service(create_appointment)
        .service(all_appointments)
        .service(modify_appointment);
}
