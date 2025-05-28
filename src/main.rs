mod cli;
mod chunk;
mod node;
mod catalog;
mod replicate;
mod compressor;
mod commands;
mod state;

use catalog::Catalog;
use clap::Parser;
use cli::{Cli, Command};
use node::Node;
use state::{load_state, save_state, State};


fn main() {
    let state = load_state();
    let args = Cli::parse();

    match args.command {
        Command::Put { path, nodes, replicas } => {
            commands::put(path, nodes, replicas);
        },

        Command::Get { name, out } => {
            println!("get");
        },

        Command::NodeLs => {
            println!("nodels")
        }
    }

    save_state();
}
