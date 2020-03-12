use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub identifier: String,
    pub owner: User,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>
}