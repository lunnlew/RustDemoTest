use clap::App;
use std::collections::HashMap;
use crate::commands::*;

pub async fn run() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut options = HashMap::new();
    if let Some(matches) = matches.subcommand_matches("unicom") {
        options.insert(String::from("user"), matches.value_of("user").unwrap().to_string());
        options.insert(String::from("password"), matches.value_of("password").unwrap().to_string());
        unicomc::run(options).await;
    }

}
