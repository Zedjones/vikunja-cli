use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "avatarUrl")]
    avatar_url: Option<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    email: Option<String>,
    id: i32,
    username: String
}