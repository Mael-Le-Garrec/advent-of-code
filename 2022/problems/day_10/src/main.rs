extern crate array_tool;
extern crate core;

use std::fs::File;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::env::args;
use std::ffi::c_char;

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

    // Store the register value
    let mut register: Vec<i32>= vec![1];

    // Read the input
    for l in reader.lines() {
        let line = l?;
        dbg!(&line);
        match line.as_str() {
            "noop" => {  // a noop lasts one instruction and does nothing -> copy the last value
                if register.len() != 0 {
                    let last_val = register[register.len() - 1];
                    register.push(last_val);
                }
            },
            _ => {
                let mut split = line.split(" ");
                let instruction = split.next().unwrap();
                let argument:i32 = split.next().unwrap().parse().unwrap();

                if instruction == "addx" {
                    let last_val = register[register.len() - 1];
                    register.push(last_val);
                    register.push(last_val + argument);
                }
                else {
                    println!("Instruction not implemented: {}", instruction);
                }
            }
        }
    }

    // Get the signal strength: position * value
    let positions: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut signal_strengths: Vec<i32> = vec![];
    for pos in &positions {
        println!("Pos: {}, X: {}", pos, &register[(*pos as usize)-1]);
        signal_strengths.push(pos * &register[(*pos as usize)-1]);
    }

    println!("Register: {:?}", register);
    println!("Signal strength sum: {:?}", signal_strengths.iter().sum::<i32>());

    // Let's print with the CRT
    // The CRT goes from 0 to 40 (wide) * 6 (high)
    // Each iteration, it draws a pixel if the value in the register is ±1
    for crt in 0..(40*6){
        let crt_i32 = crt as i32;
        if crt % 40 == 0 {
            println!();
        }
        let character = if (register[crt]-1 <= crt_i32 % 40) && (crt_i32 % 40 <= register[crt]+1) {'#'} else {'.'};
        print!("{}", character);
    }

    Ok(())
}

