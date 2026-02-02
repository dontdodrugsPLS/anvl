use clap::{Parser, SubCommand};

#[derive(Parser)]
#[command(name = "anvl", version, about)]
pub struct Args {
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short = 'y', long)]
    pub yes: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(SubCommand)]
pub enum Command {
    Init,
    Install,
    Update,
    Remove,
    Status,
    Doctor,
    List,
    Info,
    Cache,
    Config,
    Create,
    Delete,
}
