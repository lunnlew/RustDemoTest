mod auto_sign_machine;

#[macro_use]
extern crate clap;

#[tokio::main]
async fn main() {
    auto_sign_machine::run().await;
}