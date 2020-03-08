mod api;

use clap::{App, load_yaml};
use api::Client;

fn main() {
    let _yaml = load_yaml!("cli.yml");
    //let _matches = App::from(yaml).get_matches();
    let client = Client::new("https://vikunja.traphouse.us", "Zedjones", "notPass");
    println!("{:?}", client);
}
