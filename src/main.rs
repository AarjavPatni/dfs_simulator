mod cli;
mod chunk;
mod node;
mod catalog;
mod replicate;
mod compressor;
mod commands;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Put { path, name, nodes, replicas } => {
            commands::put(path, name, nodes, replicas);
        },

        Command::Get { name, out } => {
            println!("get");
        },

        Command::NodeLs => {
            println!("nodels")
        }
    }
}
