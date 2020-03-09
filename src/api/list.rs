use serde::Serialize;
use super::user::User;

#[derive(Serialize)]
pub struct List {
    created: i64,
    description: String,
    id: i32,
    identifier: String,
    owner: User,
    title: String,
    updated: i64
}