use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};

fn get_commonc_char(s1: &str, s2: &str) -> Option<char> {
    for c in s1.chars() {
        if s2.contains(c) {
            return Some(c);
        }
    }
    return None;
}

#[allow(non_snake_case)]
fn main() -> std::io::Result<()> {
    // Open the input file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    // Get the position of each letter
    let A: u32 = 'A'.into();
    let Z: u32 = 'Z'.into();
    
    let a: u32 = 'a'.into();
    let z: u32 = 'z'.into();

    // Each line is a rucksack and each character an item
    // First half of string: first compartment, second half: second
    // Goal -> find the item that is in both
    // Take their "priority" and add it to the counter
    let mut sum: u32 = 0;
    for l in reader.lines() {
        let line = l?;

        let first_half = &line[0..line.len()/2];
        let second_half = &line[line.len()/2..line.len()];
        let common_char = get_commonc_char(&first_half, &second_half).unwrap();

        let mut ord: u32 = common_char.into();

        // Numbers represent first A-Z then a-z
        // 1-26 then 27-52
        if ord > Z {
            ord = ord - a + 1; // a-z becomes first and starts at 1
        }
        else {
            ord = ord - A + 26 + 1; // A-Z becomes second (+26)
        }

        sum += ord;
        println!("{} {}", common_char, ord);
    }

    println!("Sum: {sum}");
    Ok(())
}

