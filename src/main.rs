mod cli;
mod chunk;
mod node;
mod catalog;
mod replicate;
mod compressor;

use catalog::Catalog;
use clap::Parser;
use cli::{Cli, Command};
use compressor::compress_chunk;
use node::Node;

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Put { path, name, nodes, replicas } => {
            println!("Saving #{path}...");

            println!("Chunking initiated");
            let chunks = chunk::chunk_hash_file(&path);
            println!("Chunking complete");

            let mut nodes_vec: Vec<Node> = vec![];

            // create catalog and register file
            let mut catalog = Catalog::new();
            catalog.register_file(&path);

            // create nodes
            for idx in 0..nodes {
                nodes_vec.push(Node::new(idx));
            }

            let mut cursor = 0;

            // compress and replicate chunks in nodes; add chunk to catalog
            for chunk in chunks {
                let chunk_id = chunk.id.clone();
                let osize = chunk.data.len();
                let compressed_chunk = compress_chunk(&chunk);
                let csize = compressed_chunk.data.len();
                let cratio = (csize / osize) * 100;

                println!("Original chunk size: #{osize}");
                println!("Compressed chunk {} → new size {}", chunk_id, csize);
                println!("Compression ratio = {}", cratio);

                cursor = replicate::replicate(cursor, &mut nodes_vec, replicas, &compressed_chunk);
                catalog.add_chunk(&path, chunk_id);
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
