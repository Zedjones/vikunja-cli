mod list;
mod user;
mod task;
mod label;
mod namespace;

use serde::de::DeserializeOwned;
use ureq::json;
use user::User;
use list::List;
use task::Task;
use namespace::Namespace;

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

    fn put_api_object(&self, path: &str, json: serde_json::Value) -> Result<(), String> {
        let resp = self.agent.put(&format!("{}/{}", self.server, path))
            .send_json(json);
        if resp.ok() {
            return Ok(())
        }
        let json = match resp.into_json() {
            Ok(val) => val,
            Err(err) => return Err(err.to_string())
        };
        Err(match json.get("message") {
            Some(val) => match val.as_str() {
                Some(val) => val.to_string(),
                None => "Error message from server as not a string".to_string()
            },
            None => "Error from server missing message".to_string()
        })
    }

    pub fn get_user_info(&self) -> Result<User, String> {
        self.get_api_object::<User>("user")
    }
 
    pub fn get_lists_info(&self) -> Result<Vec<List>, String> {
        self.get_api_object::<Vec<List>>("lists")
    }

    pub fn get_list_info(&self, name: &str) -> Result<Option<List>, String> {
        let mut lists = self.get_lists_info()?;
        Ok(match lists.iter().position(|list| list.title == name) {
            None => None,
            Some(list_ind) => Some(lists.swap_remove(list_ind))
        })
    }

    pub fn get_tasks_info(&self) -> Result<Vec<Task>, String> {
        self.get_api_object::<Vec<Task>>("tasks/all")
    }

    pub fn get_namespaces_info(&self) -> Result<Vec<Namespace>, String> {
        self.get_api_object::<Vec<Namespace>>("namespaces")
    }

    pub fn get_namespace_info(&self, name: &str) -> Result<Option<Namespace>, String> {
        let mut namespaces = self.get_namespaces_info()?;
        Ok(match namespaces.iter().position(|namespace| namespace.name == name) {
            None => None,
            Some(list_ind) => Some(namespaces.swap_remove(list_ind))
        })
    }

    pub fn add_task(&self, list_name: &str, title: &str) -> Result<(), String> {
        let list = self.get_list_info(list_name)?;
        match list {
            None => Ok(()),
            Some(list) => {
                self.put_api_object(&format!("lists/{}", list.id), json!({
                    "text": title
                }))
            }
        }
    }

    pub fn add_namespace(&self, title: &str) -> Result<(), String> {
        self.put_api_object("namespaces", json!({
            "name": title
        }))
    }
}