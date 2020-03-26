use super::Client;

use cursive::views::{Dialog, Button, ListView, EditView};
use cursive_async_view::AsyncView;

pub fn add_task_view(client: &mut Client, list_name: &str) -> cursive::views::Dialog {
    Dialog::new()
        .title("Add task")
        .content(
            EditView::new()
        )
}

fn handle_adding(client: &mut Client, list_name: &str, s: &mut cursive::Cursive, task: &str) {
    if task.is_empty() {
        s.add_layer(Dialog::info("Please enter a name for the task."));
    }
    else {
        
    }
}