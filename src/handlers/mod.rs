mod appointment;
mod auth;
mod contact;
mod employee;
mod note;
mod organization;
mod user;

use crate::errors::AppError;
use actix_web::{web, HttpResponse};
use appointment::{create_appointment, get_all_appointments, get_appointment, update_appointment};
use auth::auth;
use contact::{create_contact, get_all_contacts, get_contact, update_contact};
use employee::{create_employee, get_all_employees, get_employee, update_employee};
use note::{create_note, get_all_notes, get_note, update_note};
use organization::{
    create_organization, get_all_organizations, get_organization, update_organization,
};
use user::{create_user, profile, update_profile};
type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut web::ServiceConfig) {
    let organization_signup =
        web::resource("/organization-signup").route(web::post().to(create_organization));

    let all_organizations =
        web::resource("/all-organizations").route(web::get().to(get_all_organizations));

    let organization = web::resource("/organization")
        .route(web::get().to(get_organization))
        .route(web::post().to(update_organization));

    let signup = web::resource("/signup").route(web::post().to(create_user));

    let auth = web::resource("/auth").route(web::post().to(auth));

    let profile = web::resource("/profile")
        .route(web::get().to(profile))
        .route(web::post().to(update_profile));

    let all_contacts = web::resource("/contacts").route(web::get().to(get_all_contacts));
    let create_contact = web::resource("/new-contact").route(web::post().to(create_contact));

    let contact = web::resource("/contact")
        .route(web::get().to(get_contact))
        .route(web::post().to(update_contact));

    let create_appointment =
        web::resource("/create-appointment").route(web::post().to(create_appointment));

    let appointment = web::resource("/appointment")
        .route(web::post().to(update_appointment))
        .route(web::get().to(get_appointment));

    let all_appointments =
        web::resource("/all-appointments").route(web::get().to(get_all_appointments));

    let create_note = web::resource("/create-note").route(web::post().to(create_note));
    let note = web::resource("/note")
        .route(web::post().to(update_note))
        .route(web::get().to(get_note));
    let all_notes = web::resource("/all-notes").route(web::get().to(get_all_notes));

    let employee = web::resource("/employee")
        .route(web::post().to(update_employee))
        .route(web::get().to(get_employee));
    let all_employees = web::resource("/all-employees").route(web::get().to(get_all_employees));

    config
        .service(signup)
        .service(auth)
        .service(profile)
        .service(all_contacts)
        .service(create_contact)
        .service(contact)
        .service(organization_signup)
        .service(all_organizations)
        .service(organization)
        .service(create_appointment)
        .service(all_appointments)
        .service(appointment)
        .service(create_note)
        .service(note)
        .service(all_notes)
        .service(employee)
        .service(all_employees);
}
