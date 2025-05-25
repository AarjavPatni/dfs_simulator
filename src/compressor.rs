use lz4_flex::block::{compress_prepend_size, decompress_size_prepended};

use crate::chunk::Chunk;

pub fn compress_chunk(chunk: &Chunk) -> Chunk {
    Chunk {
        data: compress_prepend_size(&chunk.data),
        id: chunk.id.clone(),
        metadata: chunk.metadata.clone(),
    }
}

pub fn decompress_chunk(chunk: &Chunk) -> Chunk {
    // TODO: Handle this more gracefully
    Chunk {
        data: decompress_size_prepended(&chunk.data).unwrap(),
        id: chunk.id.clone(),
        metadata: chunk.metadata.clone(),
    }
}

