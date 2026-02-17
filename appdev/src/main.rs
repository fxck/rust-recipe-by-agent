use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;
use std::env;
use tokio_postgres::NoTls;

/// Connect to PostgreSQL and run a simple ping query to verify connectivity.
/// Returns Ok(()) on success or an error string on failure.
async fn ping_database() -> Result<(), String> {
    let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let user = env::var("DB_USER").map_err(|_| "DB_USER not set".to_string())?;
    let pass = env::var("DB_PASS").map_err(|_| "DB_PASS not set".to_string())?;
    let name = env::var("DB_NAME").unwrap_or_else(|_| "db".to_string());

    let conn_str = format!(
        "host={} port={} user={} password={} dbname={}",
        host, port, user, pass, name
    );

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls)
        .await
        .map_err(|e| format!("connection failed: {}", e))?;

    // Drive the connection in the background so the client can be used.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("PostgreSQL connection error: {}", e);
        }
    });

    // A lightweight query that every PostgreSQL version supports.
    client
        .execute("SELECT 1", &[])
        .await
        .map_err(|e| format!("ping query failed: {}", e))?;

    Ok(())
}

/// GET / — health check endpoint.
///
/// Returns HTTP 200 with JSON `{"type":"rust","status":{"database":"OK"}}`
/// when PostgreSQL is reachable, or HTTP 503 with an error detail if not.
/// This endpoint is also used as the zerops.yaml readinessCheck target so
/// Zerops waits for a healthy response before routing traffic to a new
/// runtime container.
async fn health_check() -> HttpResponse {
    match ping_database().await {
        Ok(()) => HttpResponse::Ok().json(json!({
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    println!("Starting Rust Hello World on {}", bind_addr);

    HttpServer::new(|| {
        App::new().route("/", web::get().to(health_check))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
