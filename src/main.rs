mod security;
mod query;
mod spirit;
mod config;

use actix_web::{web, App, HttpServer, HttpResponse, post};
use serde::Deserialize;
use tokio_postgres::{NoTls, Client, Error};

// Database connection function that takes parameters
async fn get_db_client(host: &str, user: &str, password: &str, dbname: &str) -> Result<Client, Error> {
    let connection_string = format!("host={} user={} password={} dbname={}", host, user, password, dbname);

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    // Spawn a new task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
    ip: String,
    db_host: String,
    db_user: String,
    db_password: String,
    db_name: String,
}

#[post("/auth")]
async fn authenticate(auth_request: web::Json<AuthRequest>) -> HttpResponse {
    // Initialize the Security instance
    let mut security = security::Security::new(5, 60);

    let input_username = &auth_request.username;
    let input_password = &auth_request.password;
    let input_ip = &auth_request.ip;

    // Check rate limiting
    if security.is_rate_limited(input_username) {
        return HttpResponse::TooManyRequests().body("Access denied: Too many attempts. Try again later.");
    }

    // Get database client using the parameters passed in the request
    let client = match get_db_client(
        &auth_request.db_host,
        &auth_request.db_user,
        &auth_request.db_password,
        &auth_request.db_name
    ).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return HttpResponse::InternalServerError().body("Failed to connect to database.");
        }
    };

    // Validate user credentials
    match query::validate_user_credentials(&client, input_username).await {
        Ok(Some((stored_hashed_password, allowed_ip))) => {
            // Check password and IP
            if security.verify_password(input_password, &stored_hashed_password)
                && security.check_ip_allowed(input_ip, &allowed_ip)
            {
                return HttpResponse::Ok().body("Access granted: Credentials and IP are valid.");
            } else {
                security.record_attempt(input_username);
                return HttpResponse::Unauthorized().body("Access denied: Invalid credentials or IP not allowed.");
            }
        }
        Ok(None) => {
            security.record_attempt(input_username);
            return HttpResponse::Unauthorized().body("Access denied: Invalid username.");
        }
        Err(e) => {
            eprintln!("Error querying database: {}", e);
            return HttpResponse::InternalServerError().body("Error querying database.");
        }
    }
}

// Call this using curl:
// curl -X POST http://127.0.0.1:8080/auth -H "Content-Type: application/json" -d '{"username": "postgres", "password": "new_password", "ip": "localhost", "db_host": "127.0.0.1", "db_user": "postgres", "db_password": "new_password", "db_name": "my_database"}'
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(authenticate) // Register the authenticate route
    })
        .bind("127.0.0.1:8080")? // Bind to an address
        .run()
        .await
}
