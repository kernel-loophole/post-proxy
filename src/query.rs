use tokio_postgres::{Client, Error};

pub async fn validate_user_credentials(
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
