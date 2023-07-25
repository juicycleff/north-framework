use poem_openapi::{payload::PlainText, OpenApi};
use poem_openapi::__private::poem::web::Data;
use crate::TestData;

#[derive(Default, Clone)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn get_index(&self, test: Data<&TestData>,) -> PlainText<String> {
        PlainText("Hello World! ".to_string() + test.title.as_str())
    }
}


#[derive(Default, Clone)]
pub struct SecondApi;

#[OpenApi]
impl SecondApi {
    #[oai(path = "/second", method = "get")]
    async fn get_index(&self, test: Data<&TestData>,) -> PlainText<String> {
        PlainText("Hello World! ".to_string() + test.title.as_str())
    }
}
