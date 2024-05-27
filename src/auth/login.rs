use actix_web::{HttpResponse, web};
use bcrypt::verify;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::auth::jwt::JwToken;
use crate::database::DB;
use crate::schema::users::{admin, email, password};
use crate::schema::users::dsl::*;

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(credentials: web::Json<Login>, mut db: DB) -> HttpResponse {
    let res: (String, i32) = match users
        .filter(email.eq(&credentials.email))
        .select((password, admin))
        .first::<(String, i32)>(&mut db.connection)
    {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::NotFound().body(e.to_string());
        }
    };

    return match verify(&credentials.password, &res.0).unwrap() {
        true => {
            let token = JwToken::new(&credentials.email, res.1).await.encode();
            HttpResponse::Ok()
                .append_header(("token", token))
                .body("Login Successful")
        }
        false => HttpResponse::Unauthorized().into(),
    };
}
