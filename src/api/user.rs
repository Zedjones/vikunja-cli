use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    #[serde(rename = "avatarUrl")]
    avatar_url: String,
    created: i64,
    updated: i64,
    email: String,
    id: i32,
    username: String
}