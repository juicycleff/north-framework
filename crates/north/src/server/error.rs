use futures::task::SpawnError;

#[derive(Debug, derive_more::Display, PartialEq, Eq)]
#[allow(dead_code)]
pub enum NorthError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    Conflict(String),
    RequestTimeout(String),
    Gone(String),
    PaymentRequired(String),
    PayloadTooLarge(String),
    TooManyRequests(String),
    DatabaseError(String),
    ParseError(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Automatically convert NorthErrors to external Response Errors
#[cfg(feature = "api-actix")]
impl ResponseError for NorthError {
    fn status_code(&self) -> StatusCode {
        match self {
            NorthError::BadRequest(_error) => StatusCode::BAD_REQUEST,
            NorthError::NotFound(_message) => StatusCode::NOT_FOUND,
            NorthError::ValidationError(_errors) => StatusCode::UNPROCESSABLE_ENTITY,
            NorthError::Unauthorized(_error) => StatusCode::UNAUTHORIZED,
            NorthError::Conflict(_message) => StatusCode::CONFLICT,
            NorthError::Gone(_errors) => StatusCode::GONE,
            NorthError::PaymentRequired(_error) => StatusCode::PAYMENT_REQUIRED,
            NorthError::PayloadTooLarge(_error) => StatusCode::PAYLOAD_TOO_LARGE,
            NorthError::TooManyRequests(_error) => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            NorthError::BadRequest(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::BadRequest().json(error.into())
            }
            NorthError::NotFound(message) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(message.into())
                // HttpResponse::NotFound().json(message.into())
            }
            NorthError::ValidationError(errors) => {
                HttpResponseBuilder::new(self.status_code()).json(errors.to_vec())
                // HttpResponse::UnprocessableEntity().json(errors.to_vec().into())
            }
            NorthError::Unauthorized(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
            }
            NorthError::Conflict(message) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(message.into())
                // HttpResponse::Conflict().json(message.into())
            }
            NorthError::Gone(errors) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(errors.into())
                // HttpResponse::Gone().json(errors.into())
            }
            NorthError::PaymentRequired(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::PaymentRequired().json(error.into())
            }
            NorthError::PayloadTooLarge(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::PayloadTooLarge().json(error.into())
            }
            NorthError::TooManyRequests(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::TooManyRequests().json(error.into())
            }
            NorthError::DatabaseError(error) => {
                HttpResponseBuilder::new(self.status_code()).body::<String>(error.into())
                // HttpResponse::TooManyRequests().json(error.into())
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

/// Utility to make transforming a string reference into an ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// Utility to make transforming a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert PoolErrors to NorthErrors
// impl From<PoolError> for NorthError {
//     fn from(error: PoolError) -> NorthError {
//         NorthError::PoolError(error.to_string())
//     }
// }

/// Convert std::io::Error to NorthErrors
impl From<std::io::Error> for NorthError {
    fn from(error: std::io::Error) -> NorthError {
        NorthError::InternalServerError(error.to_string())
    }
}

/// Convert serde_yaml::Error to NorthErrors
impl From<serde_yaml::Error> for NorthError {
    fn from(error: serde_yaml::Error) -> NorthError {
        NorthError::InternalServerError(error.to_string())
    }
}

/// Convert SpawnError to NorthErrors
impl From<SpawnError> for NorthError {
    fn from(error: SpawnError) -> NorthError {
        NorthError::InternalServerError(error.to_string())
    }
}
