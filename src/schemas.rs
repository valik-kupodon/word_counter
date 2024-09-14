use serde_derive::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Deserialize, Debug)]
pub struct ParseTextRequest {
    pub text: String,
}

#[derive(Deserialize, Debug, Clone, FromRow)]
pub struct WordCount {
    pub word: String,
    pub count: i32,
}

#[derive(Deserialize, Debug)]
pub struct WordCountRequest {
    pub words: Vec<String>,
}
