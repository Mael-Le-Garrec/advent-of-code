use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};


fn main() -> std::io::Result<()> {
    // Open the input file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let n = 14;
    for l in reader.lines() {
        let line = l?;
        println!("{}", line);

        for (i, win) in line.chars().collect::<Vec<char>>().windows(n).enumerate() {
            let mut copied_vec = Vec::new();
            copied_vec.extend_from_slice(win);

            println!("{:?}", copied_vec);
            copied_vec.sort();
            copied_vec.dedup();
            println!("  duplicates removed {:?}", copied_vec);

            if copied_vec.len() == n {  // all characters are different
                println!("{} different chars at {}", n, i+n);
                return Ok(());
            }
        }
    }

    Ok(())
}

