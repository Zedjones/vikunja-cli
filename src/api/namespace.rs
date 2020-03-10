use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::user::User;
use super::list::List;

#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace {
    created: DateTime<Utc>,
    description: String,
    id: i32,
    lists: Option<Vec<List>>,
    name: String,
    owner: User,
    updated: DateTime<Utc>
}