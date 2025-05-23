use crate::chunk::*;

pub type NodeId = u32;

pub struct Node {
    pub id: NodeId,
    pub chunks: Vec<Chunk>,
}

pub fn put_chunk(id: ChunkId, data: Chunk) -> NodeId {

}

pub fn get_chunk(id: ChunkId) -> Option<Chunk> {

}

