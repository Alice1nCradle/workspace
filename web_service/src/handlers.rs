use actix_web::{web, HttpResponse};
use super::state::AppState;
use super::db_access::*;

/**
    handlers.rs:
    tell the extractors the way to use data.
**/

pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {
    // import the information from AppState in state.rs
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    // print the conditions
    let response = format!("{} {} times", health_check_response, visit_count);

    // update visit count
    *visit_count += 1;

    //return the result by json
    HttpResponse::Ok().json(&response)
}

use super::models::Member;

pub async fn new_member(
    new_member: web::Json<Member>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Attention, received new_member!");
    let member = post_new_member_db(&app_state.db, new_member.into()).await;
    HttpResponse::Ok().json(member)
}

pub async fn get_member_via_id(
    app_state: web::Data<AppState>,
    param: web::Path<usize>,
) -> HttpResponse {
    println!("Attention, received get_member_via_id!");
    let student_id = i32::try_from(param.into_inner()).unwrap();
    let member = get_member_via_id_db(&app_state.db, student_id).await;
    HttpResponse::Ok().json(member)
}

pub async fn get_members_via_level(
    app_state: web::Data<AppState>,
    param: web::Path<usize>,
) -> HttpResponse {
    println!("Attention, received get_members_via_level!");
    let student_id = i32::try_from(param.into_inner()).unwrap();
    let members = get_members_via_level_db(&app_state.db, student_id).await;
    HttpResponse::Ok().json(members)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use crate::models::Member;
    use crate::state::AppState;
    use dotenv::dotenv;
    use std::env;
    use sqlx::postgres::PgPoolOptions;

    #[ignore]
    #[actix_web::test]
    async fn post_member_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let member = web::Json(Member {
            student_id: 2021303124,
            id: None,
            name: "Chen Shi".into(),
            level: None
        });
        let response = new_member(member, app_state).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_member_via_id_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let param: web::Path<usize> = web::Path::from(1);

        let response = get_member_via_id(app_state, param).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_members_via_level_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let param: web::Path<usize> = web::Path::from(3);

        let response = get_members_via_level(app_state, param).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}