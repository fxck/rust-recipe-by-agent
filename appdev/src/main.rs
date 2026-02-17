use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;
use std::env;
use tokio_postgres::NoTls;

/// Health check handler: connects to PostgreSQL and reports status.
/// Returns HTTP 200 with {"type":"rust","status":{"database":"OK"}} on success,
/// or HTTP 503 with error details if the database is unreachable.
async fn health_check() -> HttpResponse {
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_pass = env::var("DB_PASS").unwrap_or_else(|_| "".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "db".to_string());

    let conn_str = format!(
        "host={} port={} user={} password={} dbname={}",
        db_host, db_port, db_user, db_pass, db_name
    );

    match tokio_postgres::connect(&conn_str, NoTls).await {
        Ok((client, connection)) => {
            // Drive the connection in the background
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("PostgreSQL connection error: {}", e);
                }
            });

            // Ping with a lightweight query to confirm the connection is live
            match client.execute("SELECT 1", &[]).await {
                Ok(_) => HttpResponse::Ok().json(json!({
                    "type": "rust",
                    "status": {
                        "database": "OK"
                    }
                })),
                Err(e) => HttpResponse::ServiceUnavailable().json(json!({
                    "type": "rust",
                    "status": {
                        "database": format!("ERROR: {}", e)
                    }
                })),
            }
        }
        Err(e) => HttpResponse::ServiceUnavailable().json(json!({
            "type": "rust",
            "status": {
                "database": format!("ERROR: {}", e)
            }
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Rust hello world server on 0.0.0.0:8080");

    HttpServer::new(|| {
        App::new().route("/", web::get().to(health_check))
    })
    .bind("[::]:8080")?
    .run()
    .await
}
