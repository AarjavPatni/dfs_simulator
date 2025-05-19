pub type ChunkId = u32;

pub struct ChunkMetadata;

pub struct Chunk {
    pub data: Vec<u8>,
    pub id: u32,
    pub metadata: ChunkMetadata,
}
