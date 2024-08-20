use crate::camel_case_struct;

camel_case_struct!(PaginationParams {
     page: Option<i64>,
     per_page: Option<i64>,
});
