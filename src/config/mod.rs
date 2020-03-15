use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub server: String
}