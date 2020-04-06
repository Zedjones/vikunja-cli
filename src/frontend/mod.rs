mod task;

use super::api::{Client, FullInfo};
use task::{add_task_view, show_tasks};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use cursive::views::{Dialog, Button, ListView};
use cursive::view::IntoBoxedView as _;
use cursive::Cursive;

pub fn run_ui(client: Client) {
    let mut siv = Cursive::default();
    let client_cell = Arc::new(Mutex::from(client));
    let client_clone = client_cell.clone();

    let async_view = cursive_async_view::AsyncView::new_with_bg_creator(&mut siv, move || {
        Ok(client_clone.lock().unwrap().get_all_info().unwrap())
    }, move |full_info| Dialog::around(make_list_buttons(client_cell.clone(), full_info)).button("Quit", |s| s.quit()));
        
    siv.add_layer(async_view);
    siv.run();
}

fn make_list_buttons(client: Arc<Mutex<Client>>, all_info: FullInfo) -> ListView {
    let mut list_view = ListView::new();
    all_info.lists
        .iter()
        .map(|list| {
                let client_clone = client.clone();
                let title = Rc::new(RefCell::from(list.title.clone()));
                Button::new_raw(&list.title, move |s| {
                    match show_tasks(client_clone.clone(), title.clone()) {
                        Ok(view) => s.add_layer(view),
                        Err(err_str) => s.add_layer(Dialog::info(err_str))
                    };
                }).as_boxed_view()
            }
        )
        .for_each(|button_view| list_view.add_child("", button_view));
    list_view
}