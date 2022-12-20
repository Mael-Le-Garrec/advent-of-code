extern crate array_tool;
extern crate core;

use std::fs::File;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::env::args;
use array_tool::vec::Uniq;

fn move_head(head: &mut (i32, i32), direction: &str) {
    match direction {
        "R" => { head.0 += 1 }
        "L" => { head.0 -= 1 }
        "U" => { head.1 += 1 }
        "D" => { head.1 -= 1 }
        &_ => ()
    }
    println!("Head's position: {}, {}", head.0, head.1);
}

fn move_knots(knots: &mut Vec<(i32, i32)>) {
    // If the tail is already on the head, do nothing
    for i in 1..knots.len() {
        if &knots[i] == &knots[i-1] {
            return;
        }
        let difference = (knots[i-1].0 - knots[i].0, knots[i-1].1 - knots[i].1);
        match difference {
            // If the head is 1 step away, do nothing
            (0, 0) => (),
            (1, 1) => (),
            (1, 0) => (),
            (0, 1) => (),
            (-1, 1) => (),
            (-1, 0) => (),
            (0, -1) => (),
            (-1, -1) => (),
            (1, -1) => (),

            (x, y) if (x == 0 && y == 0) => {
                println!("what?");
                panic!();
            }

            // Otherwise, move it
            (x, y) => {
                if x == 0 {  // move vertically
                    knots[i].1 += y / y.abs();  // move of 1 but keep the sign
                } else if y == 0 {
                    knots[i].0 += x / x.abs();  // move of 1 but keep the sign
                } else {  // diagonal move
                    knots[i].0 += x / x.abs(); // move each component by 1
                    knots[i].1 += y / y.abs();
                }
            }
        }

        //println!("Tail's position: {}, {}\n", tail.0, tail.1);
    }
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

    // Head and following knots positions
    let mut knots: Vec<(i32, i32)> = vec![];
    let n_knots = 9;
    for _ in 0..(n_knots+1) {
        knots.push((0, 0));
    }
    dbg!(&knots);
    let mut positions_visited: Vec<(i32, i32)> = Vec::new();

    // Read the input
    for l in reader.lines() {
        let line = l?;
        dbg!(&line);

        let direction = line.split(" ").nth(0).unwrap();
        let steps: i32 = line.split(" ").nth(1).unwrap().parse().unwrap();

        for _ in 0..steps {
            move_head(&mut knots[0], direction);
            move_knots(&mut knots);
            positions_visited.push(knots[knots.len()-1]);
        }
    }

    //
    //println!();
    //println!("Final head position: {}, {}", head.0, head.1);
    //println!("Final tail position: {}, {}", tail.0, tail.1);
    println!("Number of unique positions visited by tail: {}", positions_visited.unique().len());
    Ok(())
}

