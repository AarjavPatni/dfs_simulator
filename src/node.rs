use std::collections::HashMap;

use crate::chunk::*;
use crate::catalog::*;

pub type NodeId = u32;

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

    pub fn put_chunk(&mut self, data: Chunk) {
        todo!()
    }

    pub fn get_chunk(&self, id: &ChunkId) -> Option<&Chunk> {
        todo!()
    }
}

