extern crate array_tool;
extern crate petgraph;
extern crate log;


use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env::args;

use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

#[allow(unused_imports)]
use log::{debug, error, warn, log_enabled, info, Level};

#[derive(Debug,Default)]
struct Node {
    character: char, // initial character
}

impl Node {
    fn get_elevation(&self) -> i32 {
        if self.character == 'E' {
            ('z' as u32 - 'a' as u32) as i32
        }
        else if self.character == 'S' {
            ('a' as u32 - 'a' as u32) as i32
        }
        else {
            (self.character as u32 - 'a' as u32) as i32
        }
    }
}

fn add_edge(
    graph: &mut Graph<Node, i32, petgraph::Directed>, 
    graph_nodes: &Vec<Vec<NodeIndex>>, 
    i: usize, 
    j: usize,
    next_i: usize,
    next_j: usize,
    jump_limit: i32)
    {
    let current_elevation = graph.node_weight(graph_nodes[i][j]).unwrap().get_elevation();
    let next_elevation = graph.node_weight(graph_nodes[next_i][next_j]).unwrap().get_elevation();

    // We can only climb +1 but we can descend more
    let difference = next_elevation - current_elevation;
    if difference <= jump_limit {
        debug!("Adding edge from {i},{j} to {next_i},{next_j}");
        graph.add_edge(graph_nodes[i][j], graph_nodes[next_i][next_j], difference);
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init();

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
    // The graph is directed because we can climb only +1 but downclimb infinitely
    let mut graph : Graph<Node, i32, petgraph::Directed> = Graph::new();
    // Also store the created nodes to retrieve them easier
    let mut graph_nodes: Vec<Vec<NodeIndex>> = vec![];

    let mut start_node_idx: (usize, usize) = (0, 0);
    let mut end_node_idx: (usize, usize) = (0, 0);

    // For problem 2, store all the starting points 'a'
    let mut vec_start: Vec<(usize, usize)> = Vec::new();

    // Read the input
    for (i, l) in reader.lines().enumerate() {
        let line = l?;
        dbg!(&line);

        let mut line_nodes: Vec<NodeIndex> = vec![];
        for (j, chr) in line.chars().enumerate() {
            if chr == 'S' {
                start_node_idx = (i, j);
                vec_start.push((i, j));
            }
            else if chr == 'E' {
                end_node_idx = (i, j);
            }
            else if chr == 'a' {
                vec_start.push((i, j));
            }
            let node = Node { character: chr };
            line_nodes.push(graph.add_node(node));
        }
        graph_nodes.push(line_nodes);
    }

    // Iterate over the map again to add or not the neighbors
    // Only add edges if the elevation difference is one
    let jump_limit = 1; // Can only move if the difference is one
    for i in 0..graph_nodes.len() {
        for j in 0..graph_nodes[0].len() {
            // Right neighbor
            if j < graph_nodes[i].len()-1 {
                add_edge(&mut graph, &graph_nodes, i, j, i, j+1, jump_limit);
            }
            // Left neighbor
            if j > 0 {
                add_edge(&mut graph, &graph_nodes, i, j, i, j-1, jump_limit);
            }
            // Up neighbor
            if i > 0 {
                add_edge(&mut graph, &graph_nodes, i, j, i-1, j, jump_limit);
            }
            // Down neighbor
            if i < graph_nodes.len()-1 {
                add_edge(&mut graph, &graph_nodes, i, j, i+1, j, jump_limit);
            }
        }
    }

    let PROBLEM = 2;

    // Get the shortest path to the end
    // Set the weight for all edges to 1 to compute the number of step
    let mut res = HashMap::<NodeIndex, i32>::new();
    if PROBLEM == 1 {
        res = dijkstra(&graph,
                                    graph_nodes[start_node_idx.0][start_node_idx.1],
                                    None,
                                |_| 1
        );

        let value = res.get(&graph_nodes[end_node_idx.0][end_node_idx.1]);
        match value {
            Some(steps) => { println!("Number of steps from start to finish: {}", steps) },
            None => { println!("Could not find a path to the end :(") },
        }
    }
    else {
        let mut min_steps = i32::MAX;
        for (i, j) in vec_start.iter() {
            println!("{i}, {j}");
            res = dijkstra(&graph,
                                        graph_nodes[*i][*j],
                                        None,
                                    |_| 1
            );
            let value = res.get(&graph_nodes[end_node_idx.0][end_node_idx.1]);
            match value {
                Some(steps) => { if steps < &min_steps {min_steps = *steps; } },
                None => { println!("Could not find a path to the end :(") },
            }
        }
        println!("Number of steps from start to finish: {}", min_steps);
    }


    //dbg!(&graph_nodes);
    //dbg!(start_node_idx);
    //dbg!(&graph_nodes[end_node_idx.0][end_node_idx.1]);
    //dbg!(&graph_nodes[start_node_idx.0][start_node_idx.1]);

    // Write teh full graph to "graph.dot" to be plotted via
    // dot -Tpng graph.dot graph.png
    let mut file = File::create("graph.dot")?;
    let result = Dot::with_config(&graph, &[Config::EdgeNoLabel]);

    match write!(file, "{:?}", result)
    {
        Ok(_) => println!("Wrote graph to graph.dot"),
        Err(_) => ()
    };


    Ok(())
}
