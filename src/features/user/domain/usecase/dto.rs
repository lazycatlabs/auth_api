use diesel::AsChangeset;
use serde::{Deserialize, Serialize};

use crate::{camel_case_struct, schema::users};

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct UpdateUserParams {
    pub name: Option<String>,
    pub photo: Option<String>,
    pub verified: Option<bool>,
}

camel_case_struct!(PaginationParams {
     page: Option<i64>,
     per_page: Option<i64>,
});
