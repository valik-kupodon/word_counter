use std::collections::HashMap;

use crate::error::JsonDecodeError;
use crate::schemas::ParseTextRequest;
use crate::services::get_text_type;
use actix_web::{error, get, post, web, Result};
use log::{error, info};
use crate::services::text_parser::StringParser;
use actix_web::web::Json;

#[get("/")]
pub async fn index() -> Result<String> {
    let body = format!("This is the index page");
    Ok(body)
}

#[get("/get_count_of_word/{word}")]
pub async fn get_count_of_word(path: web::Path<String>) -> Result<String> {
    let word = path.into_inner();
    let body = format!("Hello, {word}!");
    Ok(body)
}

#[post("/parse_text")]
pub async fn parse_text(body: web::Json<ParseTextRequest>) -> Result<Json<HashMap<String, usize>>> {
    info!("POST /parse_text {:?}", body);
    let text_type = get_text_type(body.text_type.as_str());
    if text_type.is_none() {
        let result = Err(JsonDecodeError {
            name: "Validation error on field: text_type\nerror: For text_type 'text', 'string', 'URL' available only"
        });
        error!("{:?}", result);
        return result.map_err(|err| error::ErrorBadRequest(err.name));
    }
    info!("Text type: {:?}", text_type);
    match text_type.unwrap().as_str() {
        "string" => {
            let text = body.text.as_str();
            let parser = StringParser::new(text.to_owned());
            let result = parser.get_words_count();
            info!("Result: {:?}", result);
            let result = Ok(Json(result));
            result
        }
        // "text" => {
        //     let file_path = body.text.as_str();
        //     info!("File path: {:?}", file_path);
        //     let result = Ok("File path".to_owned());
        //     result
        // }
        // "URL" => {
        //     let url = body.text.as_str();
        //     info!("URL: {:?}", url);
        //     let result = Ok("URL".to_owned());
        //     result
        // }
        _ => {
            let result = Err(JsonDecodeError {
                name: "Validation error on field: text_type\nerror: For text_type 'text', 'string', 'URL' available only"
            });
            result.map_err(|err| error::ErrorBadRequest(err.name))
        }
    }
}
