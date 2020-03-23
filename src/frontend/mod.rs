use super::api::{Client, List, FullInfo};

use cursive_async_view::AsyncView;
use cursive::views::{Dialog, TextView, Button, ListView};
use cursive::view::IntoBoxedView as _;
use cursive::Cursive;

pub fn run_ui(client: Client) {
    let mut siv = Cursive::default();
    let async_view = AsyncView::new_with_bg_creator(&mut siv, move || {
        let all_info = client.get_all_info().unwrap();

        Ok(all_info)
    }, |full_info| Dialog::around(make_list_buttons(full_info)).button("Quit", |s| s.quit()));
        
    siv.add_layer(async_view);

    siv.run();
}

fn make_list_buttons(all_info: FullInfo) -> ListView {
    let mut list_view = ListView::new();
    all_info.lists
        .iter()
        .map(|list| Button::new_raw(&list.title, |s| s.quit()).as_boxed_view())
        .for_each(|button_view| list_view.add_child("", button_view));
    list_view
}