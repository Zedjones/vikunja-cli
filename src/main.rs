mod api;

use clap::{load_yaml, Clap};
use api::Client;

#[derive(Clap)]
struct Opts {
    username: String,
    password: String,
    server: String
}

fn main() {
    let _yaml = load_yaml!("cli.yml");
    //let _matches = App::from(yaml).get_matches();
    let client = Client::new("https://vikunja.traphouse.us", "Zedjones", "notPass");
    println!("{:?}", client);
}
