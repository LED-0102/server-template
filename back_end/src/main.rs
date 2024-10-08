#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::auth::auth_config;
use crate::view::view_config;
use actix_cors::Cors;
use actix_service::Service;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

mod auth;
mod config;
mod counter;
mod database;
mod models;
mod schema;
mod view;

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    const ALLOWED_VERSION: &'static str = "v1";
    let site_counter = counter::Counter { count: 0 };
    site_counter.save().expect("Error saving the counter");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv| {
                let passed: bool;

                let mut site_counter = counter::Counter::load().unwrap();
                site_counter.count += 1;
                site_counter.save().unwrap();

                if *&req.path().contains(&format!("/{}/", ALLOWED_VERSION)) {
                    passed = true;
                } else {
                    passed = false;
                }
                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => {
                        let resp = HttpResponse::NotImplemented()
                            .body(format!("only {} API is supported", ALLOWED_VERSION));
                        Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                    }
                };
                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
            .configure(auth_config)
            .configure(view_config);
        return app;
    })
    .workers(1)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
