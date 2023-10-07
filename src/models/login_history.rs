// use chrono::{NaiveDateTime, Utc};
// use diesel::{Associations, Identifiable, Insertable, Queryable, RunQueryDsl};
// use uuid::Uuid;
//
// use crate::{
//     config::db::Connection,
//     models::user::User,
//     schema::login_history,
// };
// use crate::core::config::db::Connection;
// use crate::core::error::APIError;
// use crate::error::APIError;
//
// #[derive(Identifiable, Associations, Queryable)]
// #[belongs_to(User)]
// #[table_name = "login_history"]
// pub struct LoginHistory {
//     pub id: i32,
//     pub user_id: Uuid,
//     pub login_timestamp: NaiveDateTime,
// }
//
// #[derive(Insertable)]
// #[table_name = "login_history"]
// pub struct LoginHistoryDTO {
//     pub user_id: Uuid,
//     pub login_timestamp: NaiveDateTime,
// }
//
// impl LoginHistory {
//     pub fn create(user_id: &Uuid, conn: &mut Connection) -> Result<LoginHistoryDTO, APIError> {
//         if let Ok(user) = User::find_user_by_id(user_id, conn) {
//             let now = Utc::now().naive_utc();
//             Ok(LoginHistoryDTO {
//                 user_id: user.id,
//                 login_timestamp: now,
//             })
//         } else {
//             Err(APIError::UserNotFoundError)
//         }
//     }
//
//     pub fn save_login_history(
//         login_history_dto: LoginHistoryDTO,
//         conn: &mut Connection,
//     ) -> Result<usize, APIError> {
//         match diesel::insert_into(login_history::table)
//             .values(&login_history_dto)
//             .execute(conn)
//         {
//             Ok(data) => Ok(data),
//             Err(_) => Err(APIError::InternalError)
//         }
//     }
// }