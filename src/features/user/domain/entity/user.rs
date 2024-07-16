use uuid::Uuid;

use crate::camel_case_struct;

camel_case_struct!(UserEntity {
    id: Uuid,
    name: String,
    email: String,
    photo: String,
    verified: bool
});
