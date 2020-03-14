mod api;

use api::Client;
use serde::Deserialize;
use cursive::{Cursive, views::{Dialog, TextView}};

#[derive(Deserialize, Debug)]
struct Config {
    username: String,
    password: String,
    server: String
}

fn main() {
    dotenv::dotenv().unwrap();
    let config = envy::from_env::<Config>().unwrap();
    let client = Client::new(&config.server, &config.username, &config.password);
    let client_val = client.unwrap();
    let namespace_info = client_val.get_namespace_info("School").unwrap().unwrap();

    let mut siv = Cursive::default();
    siv.add_layer(Dialog::around(TextView::new(namespace_info.created.to_rfc2822()))
                                        .title("Namespace")
                                        .button("Quit", |s| s.quit()));

    siv.run();
}
