use crate::chunk::Chunk;

pub type NodeId = u32;

pub struct Node {
    pub id: NodeId,
    pub chunks: Vec<Chunk>,
}
