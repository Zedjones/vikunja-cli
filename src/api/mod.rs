mod list;
mod user;

use serde::{Serialize, Deserialize};
use ureq::json;
use user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    server: String,
    jwt_token: String
}

impl Client {
    pub fn new(server: &str, user: &str, pass: &str) -> Result<Client, String> {
        let api_url = format!("{}/api/v1", server);
        let resp: ureq::Response = ureq::post(&format!("{}/login", api_url))
            .set("Content-Type", "application/json")
            .send_json(json!({
                "username": user,
                "password": pass
            }));
        let res = match resp.into_json() {
            Ok(val) => val,
            Err(error) => return Err(error.to_string()),
        };
        let token = match res.get("token") {
            Some(val) => match val.as_str() {
                Some(val) => val,
                None => return  Err("JWT token was not a string".to_string())
            },
            None => return Err("Response missing JWT token".to_string())
        };
        Ok(Client{server: api_url, jwt_token: token.to_string()})
    }

    pub fn get_user_info(&self) -> Result<User, String> {
        let resp = ureq::get(&format!("{}/user", self.server))
            .set("Authorization", &format!("Bearer {}", self.jwt_token))
            .call();
        let res_str = match resp.into_string() {
            Ok(res) => res,
            Err(error) => return Err(error.to_string())
        };
        println!("{}", res_str);
        let user: User = match serde_json::from_str(&res_str) {
            Ok(user) => user,
            Err(error) => return Err(error.to_string())
        };
        return Ok(user);
    }
}