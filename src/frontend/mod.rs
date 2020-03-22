use super::api::Client;

use cursive_async_view::AsyncView;
use cursive::views::{Dialog, ScrollView, TextView};
use cursive::Cursive;

pub fn run_ui(client: Client) {
    let mut siv = Cursive::default();
    let async_view = AsyncView::new_with_bg_creator(&mut siv, move || {
        // this function is executed in a background thread, so we can block
        // here as long as we like
        let all_info = client.get_all_info().unwrap();
    
        // enough blocking, let's show the content
        Ok(format!("{:?}", all_info.tasks))
    }, |text| ScrollView::new(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit())));
        
    siv.add_layer(async_view);

    siv.run();
}