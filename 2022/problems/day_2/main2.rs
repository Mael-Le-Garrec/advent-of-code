use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};

// returns the number of points we'll get from winning or losing
// + the shape points
// X -> lose
// Y -> draw
// Z -> win
//
// A -> rock (1)
// B -> paper (2)
// C -> scisors (3)
fn get_outcome(other: &str, me: &str) -> i32
{
    match (other, me) {
        ("A", "X") => 0 + 3,
        ("B", "X") => 0 + 1,
        ("C", "X") => 0 + 2,
        
        ("A", "Y") => 3 + 1,
        ("B", "Y") => 3 + 2,
        ("C", "Y") => 3 + 3,
        
        ("A", "Z") => 6 + 2,
        ("B", "Z") => 6 + 3,
        ("C", "Z") => 6 + 1,

        (&_, _) => todo!()
    }
}

fn main() -> std::io::Result<()> {
    // Open the input file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut pts = 0;
    for l in reader.lines() {
        let line = l?;

        let moves: Vec<&str> = line.split(' ').collect();
        pts += get_outcome(moves[0], moves[1]);
    }

    println!("Max points: {pts}");

    Ok(())
}

