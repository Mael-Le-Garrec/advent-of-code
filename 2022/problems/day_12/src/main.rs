extern crate array_tool;
extern crate petgraph;

use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::env::args;

use petgraph::Graph;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn main() -> std::io::Result<()> {
    // Read the arguments
    let input: &str;
    match args().nth(1) {
        Some(e) => input = if e == "--debug" {"input2"} else {"input"},
        None => input = "input",
    }

    // Open the input file
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    // Create the graph that will be used to get the shortest path
    let mut graph : Graph<(usize, usize, char), (), petgraph::Undirected> = Graph::new_undirected();

    // Read the input
    for (i, l) in reader.lines().enumerate() {
        let line = l?;
        dbg!(&line);
        for (j, chr) in line.chars().enumerate() {
            graph.add_node((i, j, chr));
        }
    }

    dbg!(graph);

    Ok(())
}

