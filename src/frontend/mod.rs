mod task;

use super::api::{Client, FullInfo};
use task::add_task_view;

use std::cell::RefCell;
use std::rc::Rc;
use cursive::views::{Dialog, Button, ListView};
use cursive::view::IntoBoxedView as _;
use cursive::Cursive;

pub fn run_ui(client: Client) {
    let mut siv = Cursive::default();
    let client_cell = Rc::new(RefCell::from(client));
    let all_info = client_cell.borrow().get_all_info().unwrap();
        
    siv.add_layer(Dialog::around(make_list_buttons(client_cell, all_info)).button("Quit", |s| s.quit()));

    siv.run();
}

fn make_list_buttons(client: Rc<RefCell<Client>>, all_info: FullInfo) -> ListView {
    let mut list_view = ListView::new();
    all_info.lists
        .iter()
        .map(|list| {
                let client_clone = client.clone();
                let title = Rc::new(RefCell::from(list.title.clone()));
                Button::new_raw(&list.title, move |s| {
                    s.add_layer(add_task_view(client_clone.clone(), title.clone()))
                }).as_boxed_view()
            }
        )
        .for_each(|button_view| list_view.add_child("", button_view));
    list_view
}