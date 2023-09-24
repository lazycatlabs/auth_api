use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

use crate::models::response::{Diagnostic, ResponseBodyNoData};

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display(fmt = "Unauthorized.")]
    Unauthorized,
    #[display(fmt = "Invalid credentials.")]
    InvalidCredentials,
    #[display(fmt = "User does not exist.")]
    UserNotFoundError,
    #[display(fmt = "Email {} is already taken.", email)]
    EmailAlreadyExistsError { email: String },
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "{}", message)]
    BadRequest { message: String },
}

impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::EmailAlreadyExistsError { .. } => StatusCode::BAD_REQUEST,
            ServiceError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ServiceError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServiceError::UserNotFoundError => StatusCode::NOT_FOUND,
            ServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ResponseBodyNoData::new(
                Diagnostic::new(
                    &self.status_code().to_string(),
                    &self.to_string()))
            )
    }
}