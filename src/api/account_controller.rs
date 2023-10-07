// use actix_web::{HttpResponse, web};
//
// use crate::{
//     config::db::Pool,
//     constants::{*},
//     error::APIError,
//     models::{
//         response::{Diagnostic, ResponseBody, ResponseBodyNoData},
//         user::{LoginDTO, UpdateUserDTO, User, UserDTO},
//     },
//     services::account_service,
// };
//
// pub async fn signup(
//     user: web::Json<UserDTO>,
//     pool: web::Data<Pool>,
// ) -> Result<HttpResponse, APIError> {
//     match account_service::signup(user.0, &pool) {
//         Ok(_) => Ok(HttpResponse::Ok().json(
//             ResponseBodyNoData::new(
//                 Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
//             )
//         )),
//         Err(err) => Err(err)
//     }
// }
//
// pub async fn login(
//     login: web::Json<LoginDTO>,
//     pool: web::Data<Pool>,
// ) -> Result<HttpResponse, APIError> {
//     match account_service::login(login.0, &pool) {
//         Ok(token_response) => Ok(HttpResponse::Ok().json(ResponseBody::new(
//             Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
//             token_response,
//         ))),
//         Err(err) => Err(err)
//     }
// }
//
// pub async fn logout(user: User, pool: web::Data<Pool>) -> Result<HttpResponse, APIError> {
//     match account_service::logout(user.id, &pool) {
//         Ok(_) => Ok(HttpResponse::Ok().json(ResponseBodyNoData::new(
//             Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
//         ))),
//         Err(err) => Err(err)
//     }
// }
//
// pub async fn profile(user: User) -> Result<HttpResponse, APIError> {
//     Ok(HttpResponse::Ok().json(ResponseBody::new(
//         Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
//         user,
//     )))
// }
//
// pub async fn update_user(
//     user: User,
//     user_update: web::Json<UpdateUserDTO>,
//     pool: web::Data<Pool>,
// ) -> Result<HttpResponse, APIError> {
//     match account_service::update_user(user.id, user_update.0, &pool) {
//         Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new(
//             Diagnostic::new(STATUS_SUCCESS, MESSAGE_SUCCESS),
//             user,
//         ))),
//         Err(err) => Err(err)
//     }
// }
//
// pub async fn delete_user(
//     user: User,
//     pool: web::Data<Pool>,
// ) -> Result<HttpResponse, APIError> {
//     match account_service::delete_user(user.id, &pool) {
//         Ok(message) => Ok(HttpResponse::Ok().json(ResponseBodyNoData::new(
//             Diagnostic::new(STATUS_SUCCESS, message.as_str()),
//         ))),
//         Err(err) => Err(err)
//     }
// }
//
