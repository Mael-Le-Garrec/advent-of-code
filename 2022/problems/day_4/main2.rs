use std::fs::File;
#[allow(unused_imports)]
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

        // Check if one array overlaps the other
        let a = ranges[0].0;
        let b = ranges[0].1;
        let c = ranges[1].0;
        let d = ranges[1].1;

        if b >= c && a <= d {
            counter += 1;
        }
        else if d >= a && c <= b {
            counter += 1;
        }
    }

    println!("Number of overlapping arrays: {}", counter);

    Ok(())
}

