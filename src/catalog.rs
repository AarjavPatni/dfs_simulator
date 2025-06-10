use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{chunk::ChunkId, node::{Node, NodeId}};

// TODO: Add Mutex to avoid data races

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    files_to_chunks: HashMap<String, Vec<ChunkId>>,
    chunks_to_nodes: HashMap<ChunkId, Vec<NodeId>>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { files_to_chunks: HashMap::new(), chunks_to_nodes: HashMap::new() }
    }

    pub fn register_file(&mut self, filename: &String) {
        if !self.files_to_chunks.contains_key(filename) {
            self.files_to_chunks.insert(filename.clone(), Vec::new());
        }
    }

    pub fn lookup_file(&self, filename: &str) -> Option<&Vec<ChunkId>> {
        // TODO: Switch to &[ChunkId]
        self.files_to_chunks.get(filename)
    }

    pub fn add_chunk(&mut self, filename: &str, chunk_id: ChunkId, nodes_containing_chunk: Vec<NodeId>) {
        let chunk_list = self.files_to_chunks.get_mut(filename).expect("File not registered");
        chunk_list.push(chunk_id.clone());
        // TODO: Also mutate chunks_to_nodes

        // How can chunks_to_nodes be created? Everytime a chunk is stored in a node, we should
        // update the field. But when is the chunk stored in a node? When the chunk is replicated.
        // Now when is it replicated? When the replicate command from the replicate module is run.
        // But when is this run? L40 of the commands.rs file. But only replicate holds the state of
        // each chunk, i.e, where it's located. Now one way to solve this by just going through all
        // the nodes and then populating the field. But another way is to somehow return the state
        // that's modified by replicate. One design decision is that replicate only performs the
        // task of replication–given a set of nodes and replicas, it replicates the chunk across
        // the nodes. But it's not necessary that all the nodes have the chunk, so we can't assume
        // that and run add_chunk once more. We also shouldn't mutate the catalog inside replicate
        // because that breaks the one-function-one-task principle. The solution would be to return
        // the pointer to the nodes containing the chunk as a vector, in a tuple along with the cursor

        self.chunks_to_nodes.insert(chunk_id, nodes_containing_chunk);
    }

    pub fn locate_chunk(&self, chunk: &ChunkId) -> Option<&Vec<NodeId>> {
        self.chunks_to_nodes.get(chunk)
    }
}

