use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    created: DateTime<Utc>,
    created_by: User,
    description: String,
    hex_color: String,
    id: i32,
    title: String,
    updated: DateTime<Utc>
}