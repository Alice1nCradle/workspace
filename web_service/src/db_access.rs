use super::models::*;
use sqlx::postgres::PgPool;



pub async fn post_new_member_db(pool: &PgPool, new_member: Member) -> Member {
    let row = sqlx::query!(
        r#"
            INSERT INTO web_members (student_id, name, level)
            VALUES ($1, $2, 3)
            RETURNING student_id, id, name, level
        "#,
        new_member.student_id,
        new_member.name
    )
        .fetch_one(pool)
        .await
        .unwrap();

    Member {
        student_id: row.student_id,
        id: Some(row.id),
        name: row.name.clone(),
        level: Some(row.level),
    }
}



pub async fn get_member_via_id_db(pool: &PgPool, id: i32) -> Member {
    let row = sqlx::query!(
        r#"
            SELECT * FROM web_members
            WHERE id = $1
        "#,
        id
    )
        .fetch_one(pool)
        .await
        .unwrap();

    Member {
        student_id: row.student_id,
        id: Some(row.id),
        name: row.name.clone(),
        level: Some(row.level),
    }
}

pub async fn get_members_via_level_db(pool: &PgPool, level: i32) -> Vec<Member> {
    let rows = sqlx::query!(
        r#"
            SELECT * FROM web_members
            WHERE level = $1
        "#,
        level
    )
        .fetch_all(pool)
        .await
        .unwrap();

    rows.iter()
        .map(|r| Member {
            student_id: r.student_id,
            id: Some(r.id),
            name: r.name.clone(),
            level: Some(r.level),
        })
        .collect()
}