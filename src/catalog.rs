use std::collections::HashMap;

use crate::{chunk::ChunkId, node::NodeId};

// TODO: Add Mutex to avoid data races

pub struct Catalog {
    files_to_chunks: HashMap<String, Vec<ChunkId>>,
    chunks_to_nodes: HashMap<ChunkId, Vec<NodeId>>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { files_to_chunks: HashMap::new(), chunks_to_nodes: HashMap::new() }
    }

    pub fn register_file(&mut self, filename: &String) {
        if !self.files_to_chunks.contains_key(filename) {
            self.files_to_chunks.insert(filename.clone(), Vec::new());
        }
    }

    pub fn lookup_file(&self, filename: &str) -> Option<&Vec<ChunkId>> {
        // TODO: Switch to &[ChunkId]
        self.files_to_chunks.get(filename)
    }

    pub fn add_chunk(&mut self, filename: &str, chunk_id: ChunkId) {
        let chunk_list = self.files_to_chunks.get_mut(filename).expect("File not registered");
        chunk_list.push(chunk_id);
        // TODO: Also mutate chunks_to_nodes
    }

    pub fn locate_chunk(&self, chunk: &ChunkId) -> Option<&Vec<NodeId>> {
        self.chunks_to_nodes.get(chunk)
    }
}

