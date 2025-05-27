use crate::node::{NodeId, Node};
use crate::chunk;
use crate::catalog::Catalog;
use crate::compressor::compress_chunk;
use crate::replicate;

pub fn put(path: String, nodes: NodeId, replicas: usize) {
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

        let nodes_containing_chunk: Vec<NodeId>;
        (cursor, nodes_containing_chunk) = replicate::replicate(cursor, &mut nodes_vec, replicas, &compressed_chunk);
        catalog.add_chunk(&path, chunk_id, nodes_containing_chunk);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, remove_file};
    use std::io::Write;
    use std::panic;

    fn write_temp_file(path: &str, content: &[u8]) {
        let mut f = File::create(path).unwrap();
        f.write_all(content).unwrap();
    }

    #[test]
    fn test_put_single_chunk_default() {
        let path = "put_test1.txt";
        write_temp_file(path, b"hello world");
        put(path.to_string(), 3, 2);
        remove_file(path).unwrap();
    }

    #[test]
    fn test_put_empty_file() {
        let path = "put_test2_empty.txt";
        write_temp_file(path, b"");
        put(path.to_string(), 3, 2);
        remove_file(path).unwrap();
    }

    #[test]
    fn test_put_compression_ratio_prints() {
        let path = "put_test3_ratio.txt";
        write_temp_file(path, b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        put(path.to_string(), 3, 2);
        remove_file(path).unwrap();
    }

    #[test]
    fn test_put_high_replication_factor() {
        let path = "put_test4_highrep.txt";
        write_temp_file(path, b"The quick brown fox jumps over the lazy dog.");
        put(path.to_string(), 5, 5); // replicas == nodes
        remove_file(path).unwrap();
    }

    #[test]
    fn test_put_panics_if_replication_too_high() {
        let path = "put_test5_toomanyrep.txt";
        write_temp_file(path, b"small input");

        let result = panic::catch_unwind(|| {
            put(path.to_string(), 2, 5); // replicas > nodes
        });

        assert!(result.is_err(), "Expected panic when replicas > nodes");

        remove_file(path).unwrap();
    }

    #[test]
    fn test_put_multiple_chunks() {
        let path = "put_test6_multichunk.txt";
        let data = vec![b'x'; 4096]; // 4KB of data with 1KB chunk size = 4 chunks
        write_temp_file(path, &data);
        put(path.to_string(), 4, 2);
        remove_file(path).unwrap();
    }
}

