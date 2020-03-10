use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    created: DateTime<Utc>,
    description: String,
    id: i32,
    identifier: String,
    owner: User,
    title: String,
    updated: DateTime<Utc>
}