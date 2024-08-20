use dotenv_codegen::dotenv;

use crate::camel_case_struct;

// the response is used for Auth and General token
camel_case_struct!(AuthResponse {
    token: String,
    token_type: String
});

impl AuthResponse {
    pub fn new(token: String) -> Self {
        Self {
            token: format!("{}.{}", dotenv!("TOKEN_PREFIX"), token),
            token_type: "Bearer".to_string(),
        }
    }
}
