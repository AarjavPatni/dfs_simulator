mod cli;
mod chunk;
mod node;
mod catalog;
mod replicate;
mod compressor;

use clap::Parser;
use cli::{Cli, Command};
use compressor::compress_chunk;

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Put { path, name, nodes, replicas } => {
            let mut chunks = chunk::chunk_hash_file(&path);
            for chunk in chunks.iter_mut() {
                chunk.1 = compress_chunk(&chunk.1);
            }
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
