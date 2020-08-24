mod auth;
mod event;
mod user;

use crate::errors::AppError;
use actix_web::{web, HttpResponse};
use auth::auth;
use event::get_all_events;
use user::{create_user, me, update_profile};
type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut web::ServiceConfig) {
    let signup = web::resource("/signup").route(web::post().to(create_user));

    let auth = web::resource("/auth").route(web::post().to(auth));

    let me = web::resource("/me")
        .route(web::get().to(me))
        .route(web::post().to(update_profile));

    // event related
    let eventList = web::resource("/events").route(web::get().to(get_all_events));

    config.service(signup).service(auth).service(me);
}
