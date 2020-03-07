use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    println!("{:?}", matches);
}
