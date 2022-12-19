use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};

// returns who wins: -1 for opponent, 1 for me, 0 for draw
// + the number of points for the move used
// A = X -> rock (1)
// B = Y -> paper (2) 
// C = Z -> scisors (3)
fn get_outcome(other: &str, me: &str) -> (i32, i32)
{
    match (other, me) {
        ("A", "X") => (0, 1),
        ("B", "X") => (-1,1),
        ("C", "X") => (1, 1),
        
        ("A", "Y") => (1, 2),
        ("B", "Y") => (0, 2),
        ("C", "Y") => (-1,2),
        
        ("A", "Z") => (-1,3),
        ("B", "Z") => (1, 3),
        ("C", "Z") => (0, 3),

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
        let (winner, shape_pts) = get_outcome(moves[0], moves[1]);

        pts += match winner {
            -1 => 0 + shape_pts,  // lost
             0 => 3 + shape_pts,  // draw
             1 => 6 + shape_pts,  // won
            i32::MIN..=-2_i32 | 1_i32..=i32::MAX => todo!(),
        };
    }

    println!("Max points: {pts}");

    Ok(())
}

