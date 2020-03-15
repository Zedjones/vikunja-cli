mod api;
mod config;

use std::thread;
use std::time::Duration;

use api::*;
use cursive::{Cursive, views::{Dialog, TextView}};
use cursive_async_view::{AsyncView, AsyncState};
use config::Config;

fn main() {
    dotenv::dotenv().unwrap();
    let config = envy::from_env::<Config>().unwrap();
    let client = Client::new(&config.server, &config.username, &config.password);
    let client_val = client.unwrap();
    let namespace_info = client_val.get_namespace_info("School").unwrap().unwrap();

    /*
    let mut siv = Cursive::default();
    siv.add_layer(Dialog::around(TextView::new(namespace_info.created.to_rfc2822()))
                                        .title("Namespace")
                                        .button("Quit", |s| s.quit()));
    let async_view = AsyncView::new_with_bg_creator(&mut siv, move || {
        // this function is executed in a background thread, so we can block
        // here as long as we like
        thread::sleep(Duration::from_secs(10));
    
        // enough blocking, let's show the content
        Ok("Yeet! It worked 🖖")
    }, TextView::new); // create a text view from the string
    
    siv.add_layer(async_view);

    siv.run();
    */
}
