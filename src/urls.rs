use crate::error::JsonDecodeError;
use crate::schemas::ParseTextRequest;
use crate::services::get_text_type;
use actix_web::{error, get, post, web, Result};
use log::{error, info};

#[get("/get_count_of_word/{word}")]
pub async fn get_count_of_word(path: web::Path<String>) -> Result<String> {
    let word = path.into_inner();
    let body = format!("Hello, {word}!");
    Ok(body)
}

#[post("/parse_text")]
pub async fn parse_text(body: web::Json<ParseTextRequest>) -> Result<String> {
    info!("POST /parse_text {:?}", body);
    let text_type = get_text_type(body.text_type.as_str());
    if text_type.is_none() {
        let result = Err(JsonDecodeError {
            name: "For text_type 'text', 'string', 'URL' available only",
        });
        error!("{:?}", result);
        return result.map_err(|err| error::ErrorBadRequest(err.name));
    } else {
        info!("{:?}", text_type);
    }
    Ok("Text in processing".to_owned())
}
