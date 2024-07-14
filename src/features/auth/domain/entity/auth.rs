use dotenv_codegen::dotenv;

use crate::camel_case_struct;

camel_case_struct!(AuthEntity {
    token: String,
    token_type: String
});

impl AuthEntity {
    pub fn new(token: String) -> Self {
        Self {
            token: format!("{}.{}", dotenv!("TOKEN_PREFIX"), token),
            token_type: "Bearer".to_string(),
        }
    }
}
