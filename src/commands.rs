use crate::node::{NodeId, Node};
use crate::chunk;
use crate::catalog::Catalog;
use crate::compressor::compress_chunk;
use crate::replicate;

pub fn put(path: String, name: Option<String>, nodes: NodeId, replicas: usize) {
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
        let osize: f64 = chunk.data.len() as f64;
        let compressed_chunk = compress_chunk(&chunk);
        let csize: f64 = compressed_chunk.data.len() as f64;
        let cratio = ((1.0 - (csize / osize)) * 100.0) as isize;

        println!("Compressed chunk {}", chunk_id);
        println!("Change in size: {} → {}", osize, csize);
        println!("Compression ratio = {}%", cratio);
        println!();

        cursor = replicate::replicate(cursor, &mut nodes_vec, replicas, &compressed_chunk);
        catalog.add_chunk(&path, chunk_id);
    }
}
