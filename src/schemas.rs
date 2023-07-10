use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ParseTextRequest {
    pub text_type: String,
    pub text: String,
}
