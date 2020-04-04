mod task;

use super::api::{Client, FullInfo};
use task::add_task_view;

use std::sync::Arc;
use std::cell::RefCell;
use cursive_async_view::AsyncView;
use cursive::views::{Dialog, Button, ListView};
use cursive::view::IntoBoxedView as _;
use cursive::Cursive;

pub fn run_ui(client: Client) {
    let mut siv = Cursive::default();
    let client_cell = Arc::new(RefCell::from(client));
    let client_copy = client_cell.clone();
    let async_view = AsyncView::new_with_bg_creator(&mut siv, move || {
        let all_info = client_copy.borrow().get_all_info().unwrap();
        let copy = client_cell.clone();

        Ok(all_info)
    }, move |full_info| Dialog::around(make_list_buttons(client_cell, full_info)).button("Quit", |s| s.quit()));
        
    siv.add_layer(async_view);

    siv.run();
}

fn make_list_buttons(client: Rc<RefCell<Client>>, all_info: FullInfo) -> ListView {
    let mut list_view = ListView::new();
    all_info.lists
        .iter()
        .map(|list| Button::new_raw(&list.title, move |s| {
            let client_clone = client.clone();
            let title = Rc::new(RefCell::from((&list.title).clone()));
            s.add_layer(add_task_view(client_clone, title))
        }).as_boxed_view())
        .for_each(|button_view| list_view.add_child("", button_view));
    list_view
}