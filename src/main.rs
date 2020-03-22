mod api;
mod config;
mod frontend;

use api::*;
use config::Config;
use frontend::*;

fn main() {
    dotenv::dotenv().unwrap();
    let config = envy::from_env::<Config>().unwrap();
    let client = Client::new(&config.server, &config.username, &config.password);
    let client_val = client.unwrap();

    run_ui(client_val)
}