use actix_web::{get, post, web, HttpResponse, Responder};
use lapin::{options::*, BasicProperties};
use std::sync::Arc;

use crate::models::PayloadData;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/data")]
pub async fn receive_data(
    payload: web::Json<PayloadData>,
    rmq_channel: web::Data<Arc<lapin::Channel>>,
) -> impl Responder {
    let payload_bytes = match serde_json::to_vec(&payload.into_inner()) {
        Ok(bytes) => bytes,
        Err(_) => return HttpResponse::BadRequest().body("Formato JSON invalido"),
    };

    let publish_result = rmq_channel
        .basic_publish(
            "",
            "fila_dados",
            BasicPublishOptions::default(),
            &payload_bytes,
            BasicProperties::default(),
        )
        .await;

    match publish_result {
        Ok(_) => HttpResponse::Accepted().body("Payload enfileirado"),
        Err(e) => {
            eprintln!("Erro ao publicar mensagem na fila: {:?}", e);
            HttpResponse::InternalServerError().body("Erro interno do servidor")
        }
    }
}