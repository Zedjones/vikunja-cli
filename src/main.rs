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
    println!("{:?}", client.unwrap().get_user_info());
}
