extern crate array_tool;
extern crate petgraph;

use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::env::args;

use petgraph::Graph;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

#[derive(Debug,Default)]
struct Node {
    x: usize,  // position on the map
    y: usize,
    character: char, // initial character
    elevation: i32, // character translated to elevation
}

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
    let mut graph : Graph<Node, i32, petgraph::Undirected> = Graph::new_undirected();
    // Also store the created nodes to retrieve them easier
    let mut graph_nodes: Vec<Vec<NodeIndex>> = vec![];

    let mut start_node_idx: (usize, usize) = (0, 0);
    let mut end_node_idx: (usize, usize) = (0, 0);

    // Read the input
    for (i, l) in reader.lines().enumerate() {
        let line = l?;
        dbg!(&line);

        let mut line_nodes: Vec<NodeIndex> = vec![];
        for (j, chr) in line.chars().enumerate() {
            let mut elevation = (chr as i32) - ('a' as i32);
            if chr == 'S' {
                elevation = 0;
                start_node_idx = (i, j);
            }
            else if chr == 'E' {
                elevation = 25;
                end_node_idx = (i, j);
            }
            let node = Node { x: i, y: j, character: chr, elevation };
            line_nodes.push(graph.add_node(node));
        }
        graph_nodes.push(line_nodes);
    }

    // Iterate over the map again to add or not the neighbors
    // Only add edges if the elevation difference is one
    let jump_limit = 1; // Can only move if the difference is one
    for i in 0..graph_nodes.len() {
        for j in 0..graph_nodes[0].len() {
            let current_elevation = graph.node_weight(graph_nodes[i][j]).unwrap().elevation;

            // Right neighbor
            if j < graph_nodes[i].len() - 1 {
                let next_node = graph.node_weight(graph_nodes[i][j+1]).unwrap();
                let difference = (current_elevation - next_node.elevation).abs();
                if difference <= jump_limit {
                    graph.add_edge(graph_nodes[i][j], graph_nodes[i][j+1], difference);
                }
            }
            // Left neighbor
            if j > 0 {
                let next_node = graph.node_weight_mut(graph_nodes[i][j-1]).unwrap();
                let difference = (current_elevation - next_node.elevation).abs();
                if difference <= jump_limit {
                    graph.add_edge(graph_nodes[i][j], graph_nodes[i][j-1], difference);
                }
            }
            // Up neighbor
            if i > 0 {
                let next_node = graph.node_weight(graph_nodes[i-1][j]).unwrap();
                let difference = (current_elevation - next_node.elevation).abs();
                if difference <= jump_limit {
                    graph.add_edge(graph_nodes[i][j], graph_nodes[i-1][j], difference);
                }
            }
            // Down neighbor
            if i < graph_nodes.len()-1 {
                let next_node = graph.node_weight(graph_nodes[i+1][j]).unwrap();
                let difference = (current_elevation - next_node.elevation).abs();
                if difference <= jump_limit {
                    graph.add_edge(graph_nodes[i][j], graph_nodes[i+1][j], difference);
                }
            }
        }
    }


    let res = dijkstra(&graph,
                              graph_nodes[start_node_idx.0][start_node_idx.1],
                              Some(graph_nodes[end_node_idx.0][end_node_idx.1]),
                                    |x | 1
    );

    //dbg!(&graph_nodes);
    //dbg!(start_node_idx);
    //dbg!(&graph_nodes[end_node_idx.0][end_node_idx.1]);
    //dbg!(&graph_nodes[start_node_idx.0][start_node_idx.1]);

    let value = res[&graph_nodes[end_node_idx.0][end_node_idx.1]];

    println!("Number of steps from start to finish: {}", value);

    Ok(())
}

