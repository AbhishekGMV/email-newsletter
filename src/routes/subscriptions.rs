use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!", form.name)
}

pub async fn subscription(_form:web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

