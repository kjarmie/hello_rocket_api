use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppSettings {
    pub db: DBSettings
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DBSettings {
    pub username: String,
    pub password: String,
    pub port: i32,
    pub hostname: String,
    pub name: String,
    pub max_connections: i32,
}
