use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscribeFormData {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    email: String,
}

pub async fn subscribe(_: web::Form<SubscribeFormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
