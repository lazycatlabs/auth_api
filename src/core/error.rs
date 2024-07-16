use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

use crate::core::response::{Diagnostic, ResponseBody};

#[derive(Debug, Display, Error)]
pub enum APIError {
    #[display(fmt = "Unauthorized.")]
    Unauthorized,
    #[display(fmt = "The token is not intended for this application.")]
    InvalidAppCredentials,
    #[display(fmt = "Invalid credentials.")]
    InvalidCredentials,
    #[display(fmt = "User does not exist.")]
    UserNotFoundError,
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "{}", message)]
    UnauthorizedMessage { message: String },
    #[display(fmt = "{}", message)]
    BadRequest { message: String },
}

impl error::ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            APIError::UnauthorizedMessage { .. } => StatusCode::UNAUTHORIZED,
            APIError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            APIError::Unauthorized => StatusCode::UNAUTHORIZED,
            APIError::InvalidAppCredentials => StatusCode::UNAUTHORIZED,
            APIError::UserNotFoundError => StatusCode::NOT_FOUND,
            APIError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ResponseBody::<()>::new(
                Diagnostic::new(&self.status_code().as_u16().to_string(), &self.to_string()),
                None,
            ))
    }
}
