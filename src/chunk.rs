use std::{fs::File, io::BufReader, io::BufRead};
use sha2::{Sha256, Digest};

pub type ChunkId = String;

pub struct ChunkMetadata;

pub struct Chunk {
    pub data: Vec<u8>,
    pub id: ChunkId,
    pub metadata: ChunkMetadata,
}

fn main(filepath: String) -> Vec<(ChunkId, Chunk)> {
    let f = File::open(filepath).expect("Unable to open file");
    let mut reader = BufReader::new(f);
    let mut chunks: Vec<(ChunkId, Chunk)> = vec![];

    loop {
        let buf = reader.fill_buf().unwrap();
        if buf.is_empty() { break; }

        let chunk_size = buf.len().min(64);
        let file_bytes: Vec<u8> = buf[..chunk_size].to_vec();

        // TODO: Understand in-depth how hashing really works
        let chunk_hash = Sha256::digest(&file_bytes);
        let chunk_hash_str: ChunkId = format!("{:x}", chunk_hash);

        let chunk: Chunk = Chunk {
            data: file_bytes,
            id: chunk_hash_str.clone(),
            metadata: ChunkMetadata,
        };

        chunks.push((chunk_hash_str, chunk));
        reader.consume(chunk_size);
    }

    chunks
}

