use actix_web::dev::Payload;
use actix_web::error::ErrorServiceUnavailable;
use actix_web::{Error, FromRequest, HttpRequest};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use futures::future::{err, ok, Ready};
use lazy_static::lazy_static;

use crate::config::Config;

type PgPool = Pool<ConnectionManager<PgConnection>>;
pub struct DbConnection {
    pub db_connection: PgPool,
}

lazy_static! {
    static ref DBCONNECTION: DbConnection = {
        let connection_string = Config::new()
            .map
            .get("DB_URL")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        DbConnection {
            db_connection: PgPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(connection_string))
                .expect("Failed to create db connection pool"),
        }
    };
}

#[allow(dead_code)]
pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    return DBCONNECTION.db_connection.get().unwrap();
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;

    fn from_request(_: &HttpRequest, _: &mut Payload) -> Self::Future {
        return match DBCONNECTION.db_connection.get() {
            Ok(connection) => ok(DB { connection }),
            Err(_) => err(ErrorServiceUnavailable(
                "Could not make connection to database",
            )),
        };
    }
}
