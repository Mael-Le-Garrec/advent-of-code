use std::fs::File;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::io;

fn move_crate(stacks: &mut Vec<Vec<char>>, line: &str)
{
    let instructions: Vec<&str> = line.split(' ').collect();
    let number: usize = instructions[1].parse().unwrap();
    let from: usize = instructions[3].parse::<usize>().unwrap() - 1;
    let to: usize   = instructions[5].parse::<usize>().unwrap() - 1;

    let len = stacks[to].len();
    println!("Crates to move: {number}");
    for _ in 0..number {
        let crate_ = stacks[from].pop().unwrap();
        println!("  Inserting one crate ({crate_}) from {from} to {to}");
        stacks[to].insert(len, crate_);
    }
}

fn create_stacks(stacks: &mut Vec<Vec<char>>, line: &str) {
    let number_stacks = (line.len() + 1) / 4;  // 3 characters + space  
    println!("Number of stacks: {number_stacks}");

    // If the stacks vector is empty, initialize it
    if stacks.len() == 0 {
        for _ in 0..number_stacks {
            let new_vec: Vec<char> = Vec::new();
            stacks.push(new_vec);
        }
    }

    // Iterate over the stacks to get the crate
    for (i, crate_) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
        let letter: char = crate_[1];
        
        let numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        if letter != ' ' && !numbers.contains(&letter) {
            // The last element is the top of the stack
            // We're here inserting backwards
            stacks[i].insert(0, letter);  
        }
    }
    println!("Crates: {:?}", stacks);
}

fn print_top_crate(stacks: &Vec<Vec<char>>)
{
    println!("Crates at the top of each stack:");
    for crate_ in stacks {
        print!("{}", crate_[crate_.len()-1]);
    }
    io::stdout().flush().unwrap();
    println!("");
}


fn main() -> std::io::Result<()> {
    // Open the input file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    // Store our crates
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for l in reader.lines() {
        let line = l?;

        if line == "" {
            continue
        }

        // If we have instructions, perform them
        if line.starts_with("move") {
            move_crate(&mut stacks, &line);
        } else {  // otherwise that's the map and load it
            create_stacks(&mut stacks, &line);
        }
    }

    print_top_crate(&stacks);
    Ok(())
}

