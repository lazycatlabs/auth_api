use uuid::Uuid;

use crate::camel_case_struct;

camel_case_struct!(UserResponse {
    id: Uuid,
    name: String,
    email: String,
    photo: String,
    verified: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
});

camel_case_struct!(UsersResponse {
    users: Vec<UserResponse>,
    total: i64,
    page: i64,
    per_page: i64,
});
