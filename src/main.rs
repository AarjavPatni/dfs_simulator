mod cli;
mod chunk;
mod node;
mod catalog;
mod replicate;
mod compressor;

use clap::Parser;
use cli::{Cli, Command};
use compressor::compress_chunk;
use node::Node;

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Put { path, name, nodes, replicas } => {
            let mut chunks = chunk::chunk_hash_file(&path);
            let mut nodes_vec: Vec<Node> = vec![];

            for idx in 0..nodes {
                nodes_vec.push(Node::new(idx));
            }

            let mut cursor = 0;

            for chunk in chunks {
                let compressed_chunk = compress_chunk(&chunk);
                cursor = replicate::replicate(cursor, &mut nodes_vec, replicas, &compressed_chunk);
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
