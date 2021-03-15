mod auto_sign_machine;
mod utils;
mod commands;

#[macro_use]
extern crate clap;

#[tokio::main]
async fn main() {
    auto_sign_machine::run().await;
}