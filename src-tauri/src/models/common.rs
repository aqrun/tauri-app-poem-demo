use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IndexParams {
    pub name: Option<String>,
}