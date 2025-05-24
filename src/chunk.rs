use std::{fs::File, io::BufReader, io::BufRead};
use sha2::{Sha256, Digest};

pub type ChunkId = String;

#[derive(Debug)]
pub struct ChunkMetadata;

#[derive(Clone)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct Chunk {
    pub data: Vec<u8>,
    pub id: ChunkId,
    pub metadata: ChunkMetadata,
}

const CHUNK_SIZE: usize = 64;

pub fn chunk_file(filepath: &str) -> Vec<(ChunkId, Chunk)> {
    let f = File::open(filepath).expect("Unable to open file");
    let mut reader = BufReader::new(f);
    let mut chunks: Vec<(ChunkId, Chunk)> = vec![];

    loop {
        let buf = reader.fill_buf().unwrap();
        if buf.is_empty() { break; }

        let chunk_size = buf.len().min(CHUNK_SIZE);
        let file_bytes: Vec<u8> = buf[..chunk_size].to_vec();

        // TODO: Understand hashing in-depth
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, remove_file};
    use std::io::Write;

    #[test]
    fn test_chunk_file_with_short_file() {
        let path = "short_file.txt";
        let mut file = File::create(path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let chunks = chunk_file(path);

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].1.data.len(), "Hello, world!\n".len());

        remove_file(path).unwrap();
    }

    #[test]
    fn test_chunk_file_with_empty_file() {
        let path = "empty_file.txt";
        let _file = File::create(path).unwrap();

        let chunks = chunk_file(path);

        assert_eq!(chunks.len(), 0);

        remove_file(path).unwrap();
    }
}

