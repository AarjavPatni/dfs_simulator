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
        let buf = reader.fill_buf().unwrap().to_owned();
        if buf.is_empty() { break; }

        let file_bytes: Vec<u8> = if buf.len() >= 64 { buf[..64].to_vec() } else { buf[..].to_vec() };

        // TODO: Understand in-depth how hashing really works
        let chunk_hash = Sha256::digest(&file_bytes);
        let chunk_hash_str = format!("{:x}", chunk_hash);

        let chunk: Chunk = Chunk {
            data: file_bytes,
            id: chunk_hash_str.clone(),
            metadata: ChunkMetadata,
        };

        chunks.push((chunk_hash_str, chunk));

        if buf.len() >= 64 {
            reader.consume(64);
        } else {
            reader.consume(buf.len());
        }
    }

    chunks
}

