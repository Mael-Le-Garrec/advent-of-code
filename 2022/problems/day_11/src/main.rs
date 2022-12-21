extern crate array_tool;
extern crate partial_application;
extern crate num_bigint;
extern crate num_traits;

use std::fs::File;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::env::args;
use partial_application::partial;
use num_bigint::BigUint;
use num_traits::{Zero, One};

// Chose the part of the problem to solve: 1, or 2
static PROBLEM: u32 = 2;

enum CurrentStep {
    Monkey,
    Items,
    Operation,
    Test,
    True,
    False,
}

enum Operation {
    Square,
    Multiplication,
    Addition,
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

    // Store the worry level of the items taken by the monkeys
    // Monkey0: [item1, item2, ...], ...
    let mut worry_level: Vec<Vec<BigUint>> = Vec::new();

    // Each monkey has an operation to perform on the worry level
    // Let's store the functions here
    let mut operations_: Vec<Operation> = Vec::new();
    let mut operations_values: Vec<u32> = Vec::new();
    let mut mod_all: u32 = 1;  // to reduce a number, it needs to be divisible by all monkeys

    // Each monkey also has a test to perform after applying the operation
    // Those tests are always a modulo, we'll only store the number
    let mut test: Vec<u32> = Vec::new();

    // Store the monkeys the items are thrown to if the condition is true, or false
    let mut throw_true: Vec<usize> = Vec::new();
    let mut throw_false: Vec<usize> = Vec::new();

    // Store the number of times a monkey has inspected an intem
    let mut number_inspections: Vec<u64> = Vec::new();

    // Read the input
    // store the current step of the input reading: monkey, items, operation, test, true, false
    let mut current_step: CurrentStep = CurrentStep::Monkey;
    for l in reader.lines() {
        let line = l?;

        if line.is_empty() { continue };

        let sp: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
        match current_step {
            CurrentStep::Monkey => { // Nothing special to do, the monkey number is incremental
                current_step = CurrentStep::Items;
                println!("Monkey {}", &sp[1]);
            }
            CurrentStep::Items => {
                // Add a vector to the monkey list
                worry_level.push(Vec::new());
                let current_monkey = worry_level.len();

                // Add each item to the created vector
                let items_str = &sp[2..];
                for item in items_str {
                    let value: u32 = item.split(',').nth(0).unwrap().parse().unwrap();
                    let big_value: BigUint = BigUint::new(vec![value]);

                    worry_level[current_monkey-1].push(
                        big_value
                    );
                }
                println!("  Items: {:?}", worry_level[current_monkey-1]);
                number_inspections.push(0);
                current_step = CurrentStep::Operation;
            },
            CurrentStep::Operation => {
                // Figure out what the operation is. It is only new = old +/* x/old
                let operator = sp[4];
                let number: &str = sp[5];

                match (operator, number){
                    ("*", "old") => {
                        // Given the old number, square it
                        operations_.push(Operation::Square);
                        operations_values.push(0);
                    },
                    ("*", x_str) => {
                        let x: u32  = x_str.parse().unwrap();
                        operations_.push(Operation::Multiplication);
                        operations_values.push(x);

                    },
                    ("+", x_str) => {
                        let x: u32 = x_str.parse().unwrap();
                        operations_.push(Operation::Addition);
                        operations_values.push(x);
                    },
                    (&_, _) => { panic!("Operation not found") },
                }
                println!("  Operation: {} {}", operator, number);

                current_step = CurrentStep::Test;
            },
            CurrentStep::Test => {
                let number= sp[3].parse::<u32>().unwrap();
                test.push(number);
                mod_all = mod_all * number;
                println!("  Modulo {}", test[test.len()-1]);

                current_step = CurrentStep::True;
            },
            CurrentStep::True => {
                let monkey_number: usize = sp[5].parse().unwrap();
                throw_true.push(monkey_number);
                println!("  True => Monkey {}", monkey_number);

                current_step = CurrentStep::False;
            },
            CurrentStep::False => {
                let monkey_number: usize = sp[5].parse().unwrap();
                throw_false.push(monkey_number);
                println!("  False => Monkey {}\n", monkey_number);

                current_step = CurrentStep::Monkey;
            },
        }
    }

    // Now start the rounds!
    println!("\nStarting rounds");
    // Each monkey is going to inspect all of its items, and then throw them to another monkey
    let number_rounds = 10_000;
    for i in 0..number_rounds {
        for monkey in 0..worry_level.len() {  // iterate over the number of monkeys
            for _ in 0..worry_level[monkey].len() { // iterate over the worry levels
                // always take the first one as we're going to remove it later
                let worry = &worry_level[monkey][0];

                let mut result: BigUint = Zero::zero();
                match operations_[monkey] {
                    Operation::Square => {
                        result = worry * worry;
                    },
                    Operation::Multiplication => {
                        result = worry * operations_values[monkey];
                    },
                    Operation::Addition => {
                        result = worry + operations_values[monkey];
                    }
                    _ => {},
                }

                if PROBLEM == 1 {
                    result = result / (3 as u32); // relief -> divide by 3 the result
                }
                else if PROBLEM == 2 {
                    result = result % mod_all;
                }

                // Throw the item with a new worry of `result` to the other monkey
                if &result % (test[monkey]) == Zero::zero() {
                    worry_level[throw_true[monkey]].push(result);
                }
                else{
                    worry_level[throw_false[monkey]].push(result);
                }

                // Remove the thrown element
                worry_level[monkey].remove(0);

                // Increase the number of items inspected by this monkey
                number_inspections[monkey] += 1;
            }
        }
        // Let's print what each monkey holds now
        //println!("\nEnd of round {i}");
        //for monkey in 0..worry_level.len() {  // iterate over the number of monkeys
        //    println!("Monkey {}: {:?}", monkey, worry_level[monkey]);
        //}
        println!("Round {} / {}", i, number_rounds);
    }

    println!("\nInspection results:");
    for monkey in 0..worry_level.len() {  // iterate over the number of monkeys
        println!("Monkey {} inspected items {} times", monkey, number_inspections[monkey]);
    }

    let nb_monkey_business = 2; // Number of monkeys to take to calculate the business
    let mut max_times: Vec<u64> = vec![];
    for _ in 0..nb_monkey_business {
        let max_ = number_inspections.iter().max().unwrap();
        max_times.push(*max_);

        // remove it
        let index = number_inspections.iter().position(|x| &x == &max_).unwrap();
        number_inspections.remove(index);
    }

    // Finally compute the monkey business
    let mut monkey_business: u64 = 1;
    for inspections in max_times.iter() {
        monkey_business = monkey_business * (inspections);
    }
    println!("\nMonkey business: {}", monkey_business);

    Ok(())
}

