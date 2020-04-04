use super::Client;

use cursive::views::{Dialog, EditView};
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