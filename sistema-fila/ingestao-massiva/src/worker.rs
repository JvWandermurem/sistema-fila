use futures::StreamExt;
use lapin::{options::*, types::FieldTable, Connection};
use std::sync::Arc;

use crate::models::PayloadData;

pub async fn start_worker(rmq_conn: Arc<Connection>, db_pool: sqlx::PgPool) {
    println!("Iniciando worker consumidor...");
    
    let channel = rmq_conn
        .create_channel()
        .await
        .expect("Falha ao criar canal do worker");

    let mut consumer = channel
        .basic_consume(
            "fila_dados",
            "worker_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Falha ao criar consumidor");

    println!("Worker conectado e aguardando mensagens.");

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let payload_bytes = delivery.data.clone();
            
            if let Ok(dado) = serde_json::from_slice::<PayloadData>(&payload_bytes) {
                let insert_result = sqlx::query(
                    "INSERT INTO eventos (user_id, action, timestamp) VALUES ($1, $2, $3)"
                )
                .bind(&dado.user_id)
                .bind(&dado.action)
                .bind(dado.timestamp as i64)
                .execute(&db_pool)
                .await;

                match insert_result {
                    Ok(_) => {
                        let _ = delivery.ack(BasicAckOptions::default()).await;
                        println!("Registro salvo com sucesso: User {} | Action {}", dado.user_id, dado.action);
                    }
                    Err(e) => {
                        eprintln!("Erro de insercao no banco de dados: {:?}", e);
                        let _ = delivery.nack(BasicNackOptions::default()).await;
                    }
                }
            } else {
                eprintln!("Payload invalido recebido. Mensagem rejeitada.");
                let _ = delivery.reject(BasicRejectOptions::default()).await;
            }
        }
    }
}