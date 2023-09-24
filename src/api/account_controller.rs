use actix_web::{HttpResponse, web};

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        response::{Diagnostic, ResponseBody},
        user::{User, UserDTO},
    },
};

pub async fn signup(
    user: web::Json<UserDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match User::signup(user.0, &mut pool.get().unwrap()) {
        Ok(user) => Ok(HttpResponse::Ok()
            .json(
                ResponseBody::new(
                    Diagnostic::new("200", constants::SUCCESS),
                    user,
                )
            )),
        Err(e) => Err(e)
    }
}