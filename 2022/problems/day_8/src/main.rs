use std::fs::File;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::env::args;

fn print_map(map: &Vec<Vec<u32>>)
{
    for line in map.iter() {
        for c in line{
            print!("{}", c);
        }
        print!("\n");
    }
}

fn create_empty_map(size0: usize, size1: usize) -> Vec<Vec<u32>>
{
    let mut map: Vec<Vec<u32>> = Vec::new();
    for _i in 0..size0
    {
        map.push((0..size1).map(|_x| 0).collect());
    }
    map
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

    // Create the map: Vec of Vec u32
    let mut map: Vec<Vec<u32>> = Vec::new();
    for l in reader.lines() {
        let line = l?;

        // Vector of int for the line
        let line_nb: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        map.push(line_nb);
    }
    println!("Map:");
    print_map(&map);

    // Create a map of visibility: 1 = visible, 0 = non visible
    println!("\nVisibility Map:");
    let mut map_visibility = create_empty_map(map.len(), map[0].len());
    print_map(&map_visibility);

    // Check in all 4 directions if the tree can be seen
    // From left to right
    let mut latest_biggest_tree: u32 = 0;
    for (i, line) in map.iter().enumerate() {
        latest_biggest_tree = map[i][0];
        for (j, _tree) in line.iter().enumerate(){
            if j > 0 {
                if map[i][j] > latest_biggest_tree {
                    map_visibility[i][j] = 1;
                    latest_biggest_tree = map[i][j];
                }
            }
        }
    }

    // From right to left
    for (i, line) in map.iter().enumerate() {
        latest_biggest_tree = line[map[0].len() - 1];
        for (j, _tree) in line.iter().enumerate(){
            let maxlen = map[0].len() - 1;
            if maxlen - j <= map[0].len()-2 {
                if map[i][maxlen-j] > latest_biggest_tree {
                    map_visibility[i][maxlen-j] = 1;
                    latest_biggest_tree = map[i][maxlen-j];
                }
            }
        }
    }

    // From top to bottom
    for j in 0..map[0].len(){
        latest_biggest_tree = map[0][j];
        for i in 0..map.len() {
            if i > 0 {
                if map[i][j] > latest_biggest_tree {
                    map_visibility[i][j] = 1;
                    latest_biggest_tree = map[i][j];
                }
            }
        }
    }

    // From bottom to top
    for j in 0..map[0].len(){
        latest_biggest_tree = map[map.len()-1][j];
        for i in 0..map.len() {
            let maxlen = map.len() - 1;
            if maxlen - i <= map.len()-2 {
                if map[maxlen-i][j] > latest_biggest_tree {
                    map_visibility[maxlen-i][j] = 1;
                    latest_biggest_tree = map[maxlen-i][j];
                }
            }
        }
    }

    // And now the borders
   for i in 0..map.len(){
       for j in 0..map[0].len(){
           if (i == 0 || i == map.len()-1) || (j == 0 || j == map[0].len()-1){
               map_visibility[i][j] = 1;
           }
       }
   }

    println!("\nFinal Visibility Map:");
    print_map(&map_visibility);

    let mut sum = 0;
    for i in 0..map.len(){
        for j in 0..map[0].len() {
            if map_visibility[i][j] == 1 {
                sum += 1;
            }
        }
    }
    println!("Number of visible trees: {}", sum);

    Ok(())
}

