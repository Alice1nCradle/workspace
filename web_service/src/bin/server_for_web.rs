use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

// import all modules the server needs.
#[path = "../db_access.rs"] mod db_access;
#[path = "../routers.rs"] mod routers;
#[path = "../state.rs"] mod state;
#[path = "../models.rs"] mod models;
#[path = "../handlers.rs"] mod handlers;

// import all the crate I've defined.
use routers::*;
use state::AppState;

// create the http application and run it
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ready to load the database.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connect to the database.
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
    // define the data that needs to be shared to public
    // Of course, it must be the state of the application.
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK, and I 've told you".to_string(),
        visit_count: Mutex::new(0),
        // members: Mutex::new(vec![]),
        db: db_pool,
    });

    // create the application and configure it.
    let app = move || App::new()
        .app_data(shared_data.clone()) // shared_data, i
        .configure(general_routes)
        .configure(member_routes);
    // let's roll
    println!("Serving on http://localhost:8080");
    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}