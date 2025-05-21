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
    let f = File::open("log.txt").expect("Unable to open file");
    let mut reader = BufReader::new(f);
    let mut chunks: Vec<(ChunkId, Chunk)> = vec![];

    loop {
        let buf = reader.fill_buf().expect("Couldn't create buffer!");
        let mut offset = 0;
        let mut eof: bool = false;

        while !buf.is_empty() {
            let file_bytes: Vec<u8> = if buf.len() >= 64 { buf[offset..offset+64].to_vec() } else { buf[offset..].to_vec() };

            if buf.len() < 64 {
                eof = true;
            }

            // TODO: Understand in-depth how hashing really works
            let chunk_hash = Sha256::digest(&file_bytes);
            let chunk_hash_str = format!("{:x}", chunk_hash);

            let chunk: Chunk = Chunk {
                data: file_bytes,
                id: chunk_hash_str.clone(),
                metadata: ChunkMetadata,
            };

            chunks.push((chunk_hash_str, chunk));

            // TODO: fix this coz the last one may not be 64
            offset += 64;
            reader.consume(64);
        }

        if eof { break; }
    }

    chunks
}

