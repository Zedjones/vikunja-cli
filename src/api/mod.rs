mod list;
mod user;
mod task;
mod label;

use serde::de::DeserializeOwned;
use ureq::json;
use user::User;
use list::List;
use task::Task;

#[derive(Debug)]
pub struct Client {
    server: String,
    agent: ureq::Agent
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
                None => return Err("JWT token was not a string".to_string())
            },
            None => return Err("Response missing JWT token".to_string())
        };
        let mut agent = ureq::agent();
        agent.set("Authorization", &format!("Bearer {}", token));
        Ok(Client{server: api_url, agent})
    }

    fn get_api_object<T>(&self, path: &str) -> Result<T, String> where T: DeserializeOwned {
        let resp = self.agent.get(&format!("{}/{}", self.server, path))
            .call();
        let res_str = match resp.into_string() {
            Ok(res) => res,
            Err(error) => return Err(error.to_string())
        };
        let api_type: T = match serde_json::from_str(&res_str) {
            Ok(api_type) => api_type,
            Err(error) => return Err(error.to_string())
        };
        return Ok(api_type);
    }

    pub fn get_user_info(&self) -> Result<User, String> {
        self.get_api_object::<User>("user")
    }

    pub fn get_list_info(&self) -> Result<Vec<List>, String> {
        self.get_api_object::<Vec<List>>("lists")
    }

    pub fn get_tasks_info(&self) -> Result<Vec<Task>, String> {
        self.get_api_object::<Vec<Task>>("tasks/all")
    }
}