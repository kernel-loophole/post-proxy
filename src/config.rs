use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub allowed_ips: Vec<String>,
    pub user_permissions: Vec<UserPermission>,
}

#[derive(Serialize, Deserialize)]
pub struct UserPermission {
    pub username: String,
    pub schema: String,
    pub is_opt_in: bool,
}
