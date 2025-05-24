mod cli;
mod chunk;
mod node;
mod catalog;
mod replicate;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Put { path, name, nodes, replicas } => {
            let chunks = chunk::chunk_file(&path);
            println!("put");
        },

        Command::Get { name, out } => {
            println!("get");
        },

        Command::NodeLs => {
            println!("nodels")
        }
    }
}
