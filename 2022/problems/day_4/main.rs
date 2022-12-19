use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};


fn main() -> std::io::Result<()> {
    // Open the input file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut counter = 0;
    for l in reader.lines() {
        let line = l?;

        let parts: Vec<&str> = line.split(",").collect();

        let mut ranges: Vec<(i32,i32)> = Vec::new();
        for p in parts {
            let range: Vec<&str> = p.split("-").collect();
            let low: i32 = range[0].parse().unwrap();
            let high: i32 = range[1].parse().unwrap();
            ranges.push((low, high));
        }

        println!("Ranges {:?}", ranges);
        // Check if one array is contained by the other
        if ranges[0].0 >= ranges[1].0 && ranges[0].1 <= ranges[1].1 {
            counter += 1;
        }
        else if ranges[1].0 >= ranges[0].0 && ranges[1].1 <= ranges[0].1 {
            counter += 1;
        }
    }

    println!("Number of contained arrays: {}", counter);

    Ok(())
}

