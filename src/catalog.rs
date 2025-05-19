use std::collections::HashMap;

use crate::{chunk::ChunkId, node::NodeId};

pub struct Catalog {
    files_to_chunks: HashMap<String, Vec<ChunkId>>,
    chunks_to_nodes: HashMap<ChunkId, Vec<NodeId>>,
}

impl Catalog {
    pub fn register_file() {
        todo!()
    }

    pub fn lookup_file() {
        todo!()
    }

    pub fn add_chunk() {
        todo!()
    }

    pub fn locate_chunk() {
        todo!()
    }
}

