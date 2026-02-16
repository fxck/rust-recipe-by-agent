use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Serialize;
use std::env;
use tokio_postgres::NoTls;

#[derive(Serialize)]
struct HealthResponse {
    r#type: String,
    status: HealthStatus,
}

#[derive(Serialize)]
struct HealthStatus {
    database: String,
}

async fn health_check() -> HttpResponse {
    let db_status = check_database().await;

    let response = HealthResponse {
        r#type: "rust".to_string(),
        status: HealthStatus {
            database: db_status.clone(),
        },
    };

    if db_status == "OK" {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

async fn check_database() -> String {
    let db_host = match env::var("DB_HOST") {
        Ok(val) => val,
        Err(_) => return "ERROR: DB_HOST not set".to_string(),
    };

    let db_port = match env::var("DB_PORT") {
        Ok(val) => val,
        Err(_) => return "ERROR: DB_PORT not set".to_string(),
    };

    let db_user = match env::var("DB_USER") {
        Ok(val) => val,
        Err(_) => return "ERROR: DB_USER not set".to_string(),
    };

    let db_pass = match env::var("DB_PASS") {
        Ok(val) => val,
        Err(_) => return "ERROR: DB_PASS not set".to_string(),
    };

    let db_name = match env::var("DB_NAME") {
        Ok(val) => val,
        Err(_) => return "ERROR: DB_NAME not set".to_string(),
    };

    let conn_str = format!(
        "host={} port={} user={} password={} dbname={}",
        db_host, db_port, db_user, db_pass, db_name
    );

    match tokio_postgres::connect(&conn_str, NoTls).await {
        Ok((client, connection)) => {
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Database connection error: {}", e);
                }
            });

            match client.simple_query("SELECT 1").await {
                Ok(_) => "OK".to_string(),
                Err(e) => format!("ERROR: Query failed: {}", e),
            }
        }
        Err(e) => format!("ERROR: Connection failed: {}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Rust server on port 3000");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health_check))
    })
    .bind("[::]:3000")?
    .run()
    .await
}
