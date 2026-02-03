mod cli;
mod commands;
mod core;

use clap::Parser;
use cli::args::Args;

fn main() {
    let args = Args::parse();
    if let Err(e) = commands::run(args) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
