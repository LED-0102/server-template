use actix_web::web;

use crate::auth::login::login;
use crate::auth::register::register;

pub mod jwt;
pub mod login;
pub mod register;

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );
}
