// use serde::{Serialize, Deserialize};
use crate::commands::*;

pub async fn run() {
    println!("scheduler run");
    unicom::start().await;
}