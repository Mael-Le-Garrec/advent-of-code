use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};

fn get_commonc_char(s1: &str, s2: &str, s3: &str) -> Option<char> {
    for c in s1.chars() {
        if s2.contains(c) && s3.contains(c) {
            return Some(c);
        }
    }
    return None;
}

#[allow(non_snake_case)]
fn get_priority(c: char) -> u32 {
    // Get the position of each letter
    let A: u32 = 'A'.into();
    let Z: u32 = 'Z'.into();

    let a: u32 = 'a'.into();
    let _z: u32 = 'z'.into();

    // Get the ord of our char
    let ord: u32 = c.into();

    // Numbers represent first A-Z then a-z
    // 1-26 then 27-52
    if ord > Z {
        ord - a + 1 // a-z becomes first and starts at 1
    }
    else {
        ord - A + 26 + 1 // A-Z becomes second (+26)
    }
}

fn main() -> std::io::Result<()> {
    // Open the input file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    // Each line is a rucksack and each character an item
    // 3 consecutive lines is a group of elves
    // They carry only common item, that identifies them
    let mut three_lines: Vec<String> = Vec::new();
    let mut sum: u32 = 0;
    for l in reader.lines() {
        let line = l?;
        
        three_lines.push(line);

        // Get the common item when the vector is full
        if three_lines.len() == 3 {
            let c = get_commonc_char(&three_lines[0], &three_lines[1], &three_lines[2]).unwrap();
            three_lines.clear();

            let priority = get_priority(c);
            println!("Badge {} with priority {}", c, priority);
            sum += priority;
        }
    }

    println!("Sum {sum}");
    Ok(())
}

