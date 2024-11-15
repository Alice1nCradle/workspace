use actix_web::web;
use serde::{Serialize, Deserialize};

/**
    models.rs
    this module is  used to define how to manage the data from transferring.
    For example:
    Member is used to display all information from a single member.
**/


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    pub student_id: i32,
    pub id: Option<i32>,
    pub name: String,
    pub level: Option<i32>
}

impl From<web::Json<Member>> for Member {
    fn from(item: web::Json<Member>) -> Self {
        Member {
            student_id: item.student_id,
            id: item.id,
            name: item.name.clone(),
            level: item.level
        }
    }
}

