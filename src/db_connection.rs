use tokio_postgres::{Client, NoTls, Error};
use std::env;
use dotenv::dotenv;

pub async fn get_db_client() -> Result<Client, Error> {
    dotenv().ok();

    let host = env::var("DATABASE_HOST").unwrap_or_else(|_| "".to_string());
    let dbname = env::var("DATABASE_NAME").unwrap_or_else(|_| "".to_string());
    let user = env::var("DATABASE_USER").unwrap_or_else(|_| "".to_string());
    let password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "".to_string());

    let connection_string = format!(
        "host={} user={} password={} dbname={}",
        host, user, password, dbname
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
