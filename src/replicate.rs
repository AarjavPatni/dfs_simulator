use crate::chunk::*;
use crate::node::*;

pub fn replicate(mut cursor: usize, nodes: &mut Vec<Node>, mut factor: usize, chunk: &Chunk) -> usize {
    if nodes.len() < factor {
        panic!("Replication factor cannot be greater than node count");
    }

    while factor > 0 {
        let idx = cursor % &nodes.len();
        nodes[idx].chunks.insert(chunk.id.clone(), chunk.clone());
        factor -= 1;
        cursor += 1;
    }

    cursor
}

