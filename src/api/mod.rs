mod list;
mod user;
mod task;
mod label;
mod namespace;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use ureq::json;

pub use label::Label;
pub use user::User;
pub use list::List;
pub use task::Task;
pub use namespace::Namespace;

#[derive(Debug)]
pub struct Client {
    server: String,
    agent: ureq::Agent
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullInfo {
    pub labels: Vec<Label>,
    pub user: User,
    pub tasks: Vec<Task>,
    pub lists: Vec<List>,
    pub namespaces: Vec<Namespace>
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

    pub fn get_user(&self) -> Result<User, String> {
        self.get_api_object::<User>("user")
    }
 
    pub fn get_lists(&self) -> Result<Vec<List>, String> {
        self.get_api_object::<Vec<List>>("lists")
    }

    pub fn get_labels(&self) -> Result<Vec<Label>, String> {
        self.get_api_object::<Vec<Label>>("labels")
    }

    pub fn get_list_info(&self, name: &str) -> Result<Option<List>, String> {
        let mut lists = self.get_lists()?;
        Ok(match lists.iter().position(|list| list.title == name) {
            None => None,
            Some(list_ind) => Some(lists.swap_remove(list_ind))
        })
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>, String> {
        self.get_api_object::<Vec<Task>>("tasks/all")
    }

    pub fn get_namespaces(&self) -> Result<Vec<Namespace>, String> {
        self.get_api_object::<Vec<Namespace>>("namespaces")
    }

    pub fn get_namespace(&self, name: &str) -> Result<Option<Namespace>, String> {
        let mut namespaces = self.get_namespaces()?;
        Ok(match namespaces.iter().position(|namespace| namespace.name == name) {
            None => None,
            Some(list_ind) => Some(namespaces.swap_remove(list_ind))
        })
    }

    pub fn add_task(&self, list_name: &str, title: &str) -> Result<(), String> {
        match self.get_list_info(list_name)? {
            None => Err("List does not exist".to_string()),
            Some(list) => {
                self.put_api_object(&format!("lists/{}", list.id), json!({
                    "text": title
                }))
            }
        }
    }

    pub fn add_list(&self, namespace_name: &str, title: &str) -> Result<(), String> {
        match self.get_namespace(namespace_name)? {
            None => Err("Namespace does not exist".to_string()),
            Some(namespace) => {
                self.put_api_object(&format!("namespaces/{}/lists", namespace.id), json!({
                    "title": title
                }))
            }
        }
    }

    pub fn add_namespace(&self, title: &str) -> Result<(), String> {
        self.put_api_object("namespaces", json!({
            "name": title
        }))
    }

    pub fn add_label(&self, title: &str) -> Result<(), String> {
        self.put_api_object("labels", json!({
            "title": title
        }))
    }

    pub fn get_all_info(&self) -> Result<FullInfo, String> {
        crossbeam::scope(|scope| {
            let tasks_handle = scope.spawn(|_| {
                self.get_tasks()
            });
            let lists_handle = scope.spawn(|_| {
                self.get_lists()
            });
            let namespaces_handle = scope.spawn(|_| {
                self.get_namespaces()
            });
            let user_info_handle = scope.spawn(|_| {
                self.get_user()
            });
            let labels_handle = scope.spawn(|_| {
                self.get_labels()
            });
            let labels = labels_handle.join().unwrap()?;
            let tasks = tasks_handle.join().unwrap()?;
            let namespaces = namespaces_handle.join().unwrap()?;
            let lists = lists_handle.join().unwrap()?;
            let user = user_info_handle.join().unwrap()?;
            Ok(FullInfo { labels, user, lists, namespaces, tasks })
        }).unwrap()
    }
}