
use clap::App;

pub async fn run() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
 
    match matches.occurrences_of("verbose") {
        0 => println!("No verbose info"),
        1 | _ => println!("Don't be crazy"),
    }
 
    if let Some(matches) = matches.subcommand_matches("unicom") {
        if matches.is_present("user") {
            println!("Printing user info...");
        } else {
            println!("Printing normally...");
        }
    }

}