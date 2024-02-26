use crate::error::MyError;
use crate::models::teacher::*;
use sqlx::postgres::PgPool;

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query! ("SELECT id, name, picture_url, profile FROM teacher")
        .fetch_all(pool).await?;
    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone(),
        }).collect();
    
    match teachers.len() {
        0 => Err(MyError::NotFound("Not teachers found".into())),
        _ => Ok(teachers)
    }
}



pub async fn get_teacher_details_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!("SELECT id, name, picture_url, profile FROM teacher where id = $1" ,teacher_id)
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone(),
        }).map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;

    Ok(row)
}

pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        "INSERT INTO teacher (name, picture_url, profile) VALUES ($1, $2, $3) RETURNING id, name, picture_url, profile ",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile).fetch_one(pool).await?;

    Ok(Teacher {
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile,
    })
}

pub async fn update_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        "SELECT id, name, picture_url, profile FROM teacher WHERE id = $1",
        teacher_id
    ).fetch_one(pool).await
        .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;

    let temp = Teacher {
        id: row.id,
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            row.name
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            picture_url
        } else {
            row.picture_url
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            row.profile
        }
    };


    let update_row = sqlx::query!(
        "UPDATE teacher SET name = $1, picture_url = $2, profile = $3 WHERE id = $4 RETURNING id, name, picture_url, profile",
        temp.name,
        temp.picture_url,
        temp.profile,
        temp.id).fetch_one(pool).await.map(|r| Teacher {
        id: r.id,
        name: r.name,
        profile: r.profile,
        picture_url: r.picture_url
    }).map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;

    Ok(update_row)
}

pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query(&format!("DELETE FROM teacher WHERE id = {}", teacher_id))
        .execute(pool)
        .await
        .map_err(|_err| MyError::DBError("Unable to delete teacher".into()));

    Ok(format!("Deleted {:?} record", row))
}




