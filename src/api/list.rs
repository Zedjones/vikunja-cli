use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    id: i32,
    title: String,
    description: String,
    identifier: String,
    owner: User,
    created: DateTime<Utc>,
    updated: DateTime<Utc>
}