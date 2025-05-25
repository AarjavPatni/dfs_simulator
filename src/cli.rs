use clap::{Parser, Subcommand};

use crate::node::NodeId;

// Basically tells clap to fill this struct from passed args
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

// enums are converted to lowercase by clap when used as commands
// arg(long) is for marking an argument as named. eg: name is passed as --name for put
#[derive(Subcommand, Debug)]
pub enum Command {
    Put {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        nodes: NodeId,
        #[arg(long)]
        replicas: usize,
    },

    Get {
        name: String,
        #[arg(long)]
        out: Option<String>,
    },

    NodeLs,
}

