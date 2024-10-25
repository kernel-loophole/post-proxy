use tokio_postgres::{Client, Error};
use crate::query::validate_user_credentials;
use crate::security::Security;

pub struct Spirit;

impl Spirit {
    // Function to authenticate user credentials against the database
    pub async fn authenticate_user(
        client: &Client,
        username: &str,
        password: &str,
        ip: &str,
        security: &Security,
    ) -> Result<bool, Error> {
        // Validate user credentials
        match validate_user_credentials(client, username).await {
            Ok(Some((stored_hashed_password, allowed_ip))) => {
                // Check password and IP
                if security.verify_password(password, &stored_hashed_password)
                    && security.check_ip_allowed(ip, &allowed_ip)
                {
                    return Ok(true);
                } else {
                    security.record_attempt(username);
                    return Ok(false);
                }
            }
            Ok(None) => {
                security.record_attempt(username);
                return Ok(false);
            }
            Err(e) => {
                eprintln!("Error querying database: {}", e);
                return Err(e);
            }
        }
    }

    // Function to validate user credentials in the database
    async fn validate_user_credentials(
        client: &Client,
        username: &str,
    ) -> Result<Option<(String, String)>, Error> {
        let row = client
            .query_one(
                "SELECT password, allowed_ip FROM users WHERE username = $1",
                &[&username],
            )
            .await
            .ok(); // Return None if the user is not found

        match row {
            Some(row) => {
                let password: String = row.get(0);
                let allowed_ip: String = row.get(1);
                Ok(Some((password, allowed_ip)))
            }
            None => Ok(None),
        }
    }
}
