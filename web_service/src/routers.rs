use actix_web::web;
use super::handlers::*;

/**
    routers.rs:
    configures all the extractors and combines them with handlers.
**/

/**
    Caution:
    I've already put the error check in the handlers.rs, so no need to write in routers.rs
**/

// the route about health check
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}


// all the routes about members' information
pub fn member_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/members")
        .route("/", web::post().to(new_member))
        .route("/{member_id}", web::get().to(get_member_via_id))
        .route("/level/{level}", web::get().to(get_members_via_level)));
}


