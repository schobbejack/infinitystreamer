use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    pub start: Option<String>,
    pub end: Option<String>,
}
