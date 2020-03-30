use super::Client;

use cursive::views::{Dialog, Button, ListView, EditView};
//use cursive_async_view::AsyncView;

pub fn add_task_view(client: &mut Client, list_name: &str) -> cursive::views::Dialog {
    Dialog::new()
        .title("Add task")
        .content(
            EditView::new()
                    .on_submit(|s, text| handle_adding(client, list_name, s, text))
        )
}

fn handle_adding(client: &mut Client, list_name: &str, s: &mut cursive::Cursive, task: &str) {
    if task.is_empty() {
        s.add_layer(Dialog::info("Please enter a name for the task."));
    }
    else {
        match client.add_task(list_name, task) {
            Err(error) => s.add_layer(Dialog::info(error)),
            _ => s.add_layer(Dialog::info(format!("Successfully added task to {}", list_name)))
        };
    }
}