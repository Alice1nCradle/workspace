use std::sync::Mutex;
use sqlx::postgres::PgPool;

// Maybe the RustRover IDE will import the module I want automatically.
// I need to import my CET=6 grade.
/**
    state.rs:
    Here needs a struct to describe the state of the application.
    For example:
    1. health_check_response, a message to tell administrator whether the server is alive.
    2. visit_count, calculate how many times have this site been seen.
    3. db, based database
**/

/**
    Caution:
    As for the thread competition, we can easily know that there'll be so many visits.
    And we need to avoid deadlock or thread panicked.
    So it's of significance to import **Mutex** or **Arc**.
**/
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u64>,
    //pub members: Mutex<Vec<Member>>,
    pub db: PgPool,
}