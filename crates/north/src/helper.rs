#![allow(dead_code)]

#[cfg(feature = "api-poem")]
use poem::web::Json as PoemJson;
#[cfg(feature = "api-poem")]
use poem_openapi::{
    payload::{Json, PlainText},
    types::{ParseFromJSON, ToJSON},
    ApiRequest, ApiResponse,
};

use serde::Serialize;

use crate::utils::server_utils::NorthResult;

/// Helper function to reduce boilerplate of an OK/Json response
#[cfg(feature = "api-actix")]
pub fn respond_json<T>(data: T) -> NorthResult<Json<T>>
where
    T: Serialize,
{
    Ok(Json(data))
}

#[cfg(feature = "api-poem")]
pub fn respond_json<T>(data: T) -> NorthResult<PoemJson<T>>
where
    T: Serialize,
{
    Ok(PoemJson(data))
}

/// Helper function to reduce boilerplate of an empty OK response
#[cfg(feature = "api-actix")]
pub fn respond_ok() -> NorthResult<HttpResponse> {
    Ok(HttpResponseBuilder::new(StatusCode::OK).body::<String>("".to_string()))
}

/// ## RequestObject
/// Generic helper enum for wrapping API request dto objects
#[cfg(feature = "api-poem")]
#[derive(ApiRequest)]
pub enum RequestObject<T: Send + ToJSON + ParseFromJSON> {
    #[oai()]
    Input(Json<T>),
}

/// ## ResponseObject
/// Helper enum for wrapping API response dto objects
#[cfg(feature = "api-poem")]
#[derive(ApiResponse)]
pub enum ResponseObject<T: Send + ToJSON> {
    #[oai(status = 200)]
    Ok(Json<T>),

    #[oai(status = 202)]
    Accepted(Json<T>),

    #[oai(status = 201)]
    Created(Json<T>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),

    #[oai(status = 401)]
    Unauthorized(PlainText<String>),

    #[oai(status = 402)]
    PaymentRequired(PlainText<String>),

    #[oai(status = 403)]
    Forbidden(PlainText<String>),

    #[oai(status = 405)]
    MethodNotAllowed(PlainText<String>),

    #[oai(status = 406)]
    NotAcceptable(PlainText<String>),

    #[oai(status = 407)]
    ProxyAuthenticationRequired(PlainText<String>),

    #[oai(status = 408)]
    RequestTimeout(PlainText<String>),

    #[oai(status = 409)]
    Conflict(PlainText<String>),

    #[oai(status = 410)]
    Gone(PlainText<String>),

    #[oai(status = 411)]
    LengthRequired(PlainText<String>),

    #[oai(status = 412)]
    PreconditionFailed(PlainText<String>),

    #[oai(status = 413)]
    PayloadTooLarge(PlainText<String>),

    #[oai(status = 414)]
    URITooLong(PlainText<String>),

    #[oai(status = 415)]
    UnsupportedMediaType(PlainText<String>),

    #[oai(status = 422)]
    UnprocessableEntity(PlainText<String>),

    #[oai(status = 429)]
    TooManyRequests(PlainText<String>),

    #[oai(status = 431)]
    RequestHeaderFieldsTooLarge(PlainText<String>),

    #[oai(status = 451)]
    UnavailableForLegalReasons(PlainText<String>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct TestResponse {
        pub first_name: String,
    }

    #[cfg(feature = "api-poem")]
    #[test]
    fn it_responds_json() {
        let response = TestResponse {
            first_name: "satoshi".into(),
        };
        let result = respond_json(response.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().first_name, response.first_name);
    }

    #[cfg(feature = "api-actix")]
    #[test]
    fn it_responds_ok() {
        let result = respond_ok();
        assert!(result.is_ok());
    }
}
