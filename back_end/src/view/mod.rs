use actix_web::web;
pub mod hello_world;
pub fn view_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("v1").route("/hello-world", web::get().to(hello_world::hello_world)));
}
