use std::{fs::File, io::{BufReader, BufWriter}};

use crate::catalog::Catalog;
use crate::node::Node;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct State {
    catalog: Catalog,
    nodes: Vec<Node>
}

impl State {
    pub fn new() -> State {
        State {
            catalog: Catalog::new(),
            nodes: vec![],
        }
    }
}


pub fn load_state() -> State {
    // 1. create a new struct
    // 2. deserialize catalog.bin
    // 3. run a for loop through all the nodes and add them to the State

    let file = File::open("state.bin").unwrap();
    let reader = BufReader::new(file);
    let local_state: State = bincode::deserialize_from(reader).unwrap();

    let mut state: State = State::new();

    for node in local_state.nodes {
        &state.nodes.push(node);
    }

    state
}

pub fn save_state(state: State) {
    // 1. save the state into the file structure

    let file = File::create("state.bin").unwrap();
    let writer = BufWriter::new(file);

    bincode::serialize_into(writer, &state).unwrap();
}

