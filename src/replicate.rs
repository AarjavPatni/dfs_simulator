use crate::chunk::*;
use crate::node::*;

pub fn replicate(cursor: usize, nodes: &mut Vec<Node>, factor: usize, chunk: &Chunk) -> usize {
    if nodes.len() < factor {
        // TODO: Switch out panic! for a more graceful error handling
        panic!("Replication factor cannot be greater than node count");
    }

    let mut current_cursor = cursor;
    let mut current_factor = factor;

    while current_factor > 0 {
        let idx = cursor % nodes.len();
        nodes[idx].chunks.insert(chunk.id.clone(), chunk.clone());
        current_factor -= 1;
        current_cursor += 1;
    }

    current_cursor
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::{Chunk, ChunkMetadata};

    /// Helper to build a dummy chunk with a given ID.
    fn make_chunk(id: &str) -> Chunk {
        Chunk {
            data: vec![0; 4],
            id: id.to_string(),
            metadata: ChunkMetadata,
        }
    }

    #[test]
    fn replicate_round_robin_basic() {
        // Three nodes, replicate factor 2, starting at cursor 0
        let mut nodes = vec![Node::new(1), Node::new(2), Node::new(3)];
        let chunk = make_chunk("c1");
        let new_cursor = replicate(0, &mut nodes, 2, &chunk);
        // Cursor should advance by 2
        assert_eq!(new_cursor, 2);
        // First two nodes should have the chunk
        assert!(nodes[0].chunks.contains_key("c1"));
        assert!(nodes[1].chunks.contains_key("c1"));
        // Third node should not
        assert!(!nodes[2].chunks.contains_key("c1"));
    }

    #[test]
    fn replicate_wraps_around() {
        // Two nodes, replication factor = 2 (== node count), start at cursor=1
        let mut nodes = vec![Node::new(10), Node::new(20)];
        let chunk = make_chunk("x");
        // We expect indices: 1 % 2 = 1, (1+1) % 2 = 0
        let new_cursor = replicate(1, &mut nodes, 2, &chunk);
        // Cursor advances by factor (1 + 2 = 3)
        assert_eq!(new_cursor, 3);

        // Both nodes must have received the chunk once
        assert!(nodes[1].chunks.contains_key("x"));
        assert!(nodes[0].chunks.contains_key("x"));
    }

    #[test]
    #[should_panic(expected = "Replication factor cannot be greater than node count")]
    fn replicate_panics_if_factor_gt_nodes() {
        // Only one node, factor=2 should panic
        let mut nodes = vec![Node::new(42)];
        let chunk = make_chunk("z");
        let _ = replicate(0, &mut nodes, 2, &chunk);
    }
}

