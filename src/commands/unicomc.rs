use std::collections::HashMap;
use crate::utils::scheduler;

pub async fn run(options: HashMap<String, String>) {

    println!("unicomc run {} {}", options.get("user").unwrap().to_string(), options.get("password").unwrap().to_string());
    scheduler::run().await;
    
}