use actix_web::{web, HttpResponse};
use sqlx::{Connection, PgConnection};

use crate::configuration::get_configuration;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("email: {}, name: {}", form.email, form.name);
    HttpResponse::Ok().finish()
}
