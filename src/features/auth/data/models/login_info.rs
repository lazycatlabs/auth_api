use uuid::Uuid;

use crate::camel_case_struct;

camel_case_struct!(LoginInfo {
    id: String,
    email: String,
    login_session: Uuid
});
