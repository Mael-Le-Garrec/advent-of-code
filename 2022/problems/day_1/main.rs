use std::fs::File;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};


fn main() -> std::io::Result<()> {
    // Open the input file
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    // Create the elves hashmap and populate it
    let mut elves: HashMap<i32, Vec<i32>> = HashMap::new();  // {elf1: Vec, elf2: Vec}
    let mut elf_number = 0;
    for l in reader.lines() {
        // Add the vec if it's the firs time seeing this elf
        let line = l?;
        if !elves.contains_key(&elf_number)
        {
            let mut vec = Vec::new();
            vec.push(line.parse().unwrap());
            elves.insert(elf_number, vec);
        }
        else if line == "" {
            elf_number += 1;
        }
        else {  // add the number to the vector
            elves.get_mut(&elf_number).map(|val| val.push(line.parse().unwrap()));
        }
    }

    // Find the vector with the highest sum
    let mut max = 0;
    for elf in elves.keys() {
        let sum = elves[elf].iter().sum();
        if sum > max {
            max = sum;
        }

        //println!("{:?}", elves[elf]);
    }
    println!("Max calories carried by one elf: {}", max);


    // Find the top three elves calories
    let mut sum_elves: Vec<i32> = Vec::new();
    for elf in elves.keys() {
        sum_elves.push(elves[elf].iter().sum());
    }

    let l = sum_elves.len();
    sum_elves.sort();
    sum_elves.reverse();

    //println!("{:?}", &sum_elves[0..3]);
    println!("Sum of the max 3 elves: {}", &sum_elves[0..3].iter().sum::<i32>());

    Ok(())
}
