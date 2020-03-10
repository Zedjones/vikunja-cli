mod api;

use api::Client;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    username: String,
    password: String,
    server: String
}

fn main() {
    dotenv::dotenv().unwrap();
    let config = envy::from_env::<Config>().unwrap();
    println!("{:?}", config);
    let client = Client::new(&config.server, &config.username, &config.password);
    println!("{:?}", client);
    let client_val = client.unwrap();
    println!("{:?}", client_val.get_tasks_info());
}
