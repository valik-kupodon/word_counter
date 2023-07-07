use actix_web::{get, post, HttpResponse, Responder};

#[get("/get_count_of_word")]
pub async fn get_count_of_word() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/parse_text")]
pub async fn parse_text(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
