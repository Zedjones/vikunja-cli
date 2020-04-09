use super::Client;

use cursive::views::{Dialog, EditView, ListView, Button};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
//use cursive_async_view::AsyncView;

pub fn add_task_view(client: Arc<Mutex<Client>>, list_name: Rc<RefCell<String>>) -> cursive::views::Dialog {
    Dialog::new()
        .title("Add task")
        .content(
            EditView::new()
                    .on_submit(move |s, text| handle_adding(client.clone(), list_name.clone(), s, text))
        )
        .button("Cancel", |s| { s.pop_layer(); })
}

fn handle_adding(client: Arc<Mutex<Client>>, list_name: Rc<RefCell<String>>, s: &mut cursive::Cursive, task: &str) {
    if task.is_empty() {
        s.add_layer(Dialog::info("Please enter a name for the task."));
    }
    else {
        match client.lock().unwrap().add_task(&list_name.borrow(), task) {
            Err(error) => s.add_layer(Dialog::info(error)),
            _ => {
                s.pop_layer();
                s.add_layer(Dialog::info(format!("Successfully added task to {}", &list_name.borrow())))
            }
        };
    }
}


pub fn show_tasks(client: Arc<Mutex<Client>>, list_name: Rc<RefCell<String>>) -> Result<cursive::views::Dialog, String> {
    let tasks = client.lock().unwrap().get_tasks_of_list(&list_name.borrow())?;
    let mut list_view = ListView::new();
    tasks
        .iter()
        .map(|task| {
            Button::new_raw(&task.text, |_s| { })
        })
        .for_each(|button_view| list_view.add_child("", button_view));
    let dialog = 
        Dialog::around(list_view)
        .button("Add", move |s| {
            let client_clone = client.clone();
            let list_name_clone = list_name.clone();
            s.add_layer(add_task_view(client_clone, list_name_clone))
        })
        .button("Close", |s| { s.pop_layer(); });
    Ok(dialog)
}