mod config;
mod file;
mod process;
mod system;
mod network;
mod services;
mod output;

fn main() {
    println!("Hello, world!");
    config::init();
    file::init();
    process::init();
    system::init();
    network::init();
    services::init();
    output::init();
}
