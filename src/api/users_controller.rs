use actix_web::{get, Responder, web};

use crate::config::db::PostgresPool;
use crate::errors::user::ServiceError;
use crate::models::user::User;

#[get("/api/users")]
async fn find_all(
    pool: web::Data<PostgresPool>,
) -> Result<impl Responder, ServiceError> {
    let users = web::block(move || User::get_all(&mut pool.get().unwrap()))
        .await.unwrap()?;

    Ok(web::Json(users))
}