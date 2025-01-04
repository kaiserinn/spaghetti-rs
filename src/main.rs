use spaghetti_rs::{cli, config::Config};
use clap::Parser;

fn main() {
    let config = Config::parse();

    cli::run(&config);
}
