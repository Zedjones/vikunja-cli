use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::user::User;
use super::list::List;

#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace {
    pub created: DateTime<Utc>,
    pub description: String,
    pub id: i32,
    pub lists: Option<Vec<List>>,
    pub name: String,
    pub owner: User,
    pub updated: DateTime<Utc>
}