use serde::{Serialize, Deserialize};
use ureq::json;

#[derive(Serialize, Deserialize)]
pub struct Client {
    server: String,
    jwtToken: String
}

impl Client {
    pub fn new(server: &str, user: &str, pass: &str) -> Result<Client, String> {
        let api_url = format!("{}/api/v1/", server);
        let resp: ureq::Response = ureq::post(&format!("{}/login", api_url))
            .set("Content-Type", "application/json")
            .send_json(json!({
                "username": user,
                "password": pass
            }));
        let json = resp.into_json();
    }
}