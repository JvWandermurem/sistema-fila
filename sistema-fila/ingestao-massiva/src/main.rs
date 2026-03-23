mod models;
mod routes;
mod worker;

#[cfg(test)]
mod tests;

use actix_web::{web, App, HttpServer};
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::env;

use routes::{health_check, receive_data};
use worker::start_worker;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Inicializando pool de conexoes com o banco de dados...");
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:password123@127.0.0.1:5433/massivedb".to_string());
    
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Falha ao conectar no PostgreSQL");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS eventos (
            id SERIAL PRIMARY KEY,
            user_id VARCHAR(255) NOT NULL,
            action VARCHAR(255) NOT NULL,
            timestamp BIGINT NOT NULL
        );"
    )
    .execute(&db_pool)
    .await
    .expect("Falha na criacao da tabela");

    println!("Inicializando conexao com o RabbitMQ...");
    let addr = env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://admin:password123@127.0.0.1:5672/%2f".to_string());
    
    let conn = Connection::connect(&addr, ConnectionProperties::default())
        .await
        .expect("Falha ao conectar no RabbitMQ");

    let rmq_conn_arc = Arc::new(conn);
    let channel = rmq_conn_arc
        .create_channel()
        .await
        .expect("Falha ao criar canal no RabbitMQ");
    
    channel
        .queue_declare(
            "fila_dados",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Falha ao declarar fila");

    let worker_conn = rmq_conn_arc.clone();
    let worker_pool = db_pool.clone();
    
    tokio::spawn(async move {
        start_worker(worker_conn, worker_pool).await;
    });

    let shared_channel = Arc::new(channel);

    println!("Servidor HTTP ativo em http://0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_channel.clone()))
            .service(health_check)
            .service(receive_data)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}