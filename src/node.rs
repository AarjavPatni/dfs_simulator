use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::chunk::*;
use crate::catalog::*;

pub type NodeId = u32;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub chunks: HashMap<ChunkId, Chunk>,
}

impl Node {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            chunks: HashMap::new(),
        }
    }

    pub fn put_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(chunk.id.clone(), chunk);
    }

    pub fn get_chunk(&self, id: &ChunkId) -> Option<&Chunk> {
        self.chunks.get(id)
    }
}

