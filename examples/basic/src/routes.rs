use poem_openapi::{payload::PlainText, OpenApi};

#[derive(Default, Clone)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn get_index(&self) -> PlainText<String> {
        PlainText("Hello World!".to_string())
    }
}
