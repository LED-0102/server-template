use crate::config::Config;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use chrono::{TimeDelta, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub exp: usize,
    pub email: String,
    pub admin: i32,
}
impl JwToken {
    pub fn get_key() -> String {
        let config = Config::new();
        let key_str = config.map.get("SECRET_KEY").unwrap().as_str().unwrap();
        return key_str.to_owned();
    }
    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        let token = encode(&Header::default(), &self, &key).unwrap();
        return token;
    }
    pub async fn new(email_id: &String, admin_id: i32) -> Self {
        let config = Config::new();
        let minutes = config.map.get("EXPIRE_MINUTES").unwrap().as_i64().unwrap();
        let timestamp = Utc::now()
            .checked_add_signed(TimeDelta::try_minutes(minutes).unwrap())
            .expect("valid Timestamp")
            .timestamp();
        return JwToken {
            exp: timestamp as usize,
            email: email_id.clone(),
            admin: admin_id,
        };
    }
    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        let token_result = decode::<JwToken>(&token, &key, &Validation::default());
        match token_result {
            Ok(data) => Ok(data.claims),
            Err(error) => {
                let message = format!("{}", error);
                return Err(message);
            }
        }
    }
}
impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwToken::from_token(raw_token);
                match token_result {
                    Ok(token) => ok(token),
                    Err(message) => {
                        if message == "ExpiredSignature".to_owned() {
                            return err(ErrorUnauthorized("Token expired"));
                        }
                        return err(ErrorUnauthorized("Token can't be decoded"));
                    }
                }
            }
            None => {
                return err(ErrorUnauthorized("token not in header under key 'token'"));
            }
        }
    }
}
