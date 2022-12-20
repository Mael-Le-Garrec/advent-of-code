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

fn get_scenic_score(pos_y: usize, pos_x: usize, map: &Vec<Vec<u32>>) -> u32
{
    // Go in all directions from the given point until we reach a tree of the same size or bigger
    let mut score = [0, 0, 0, 0];  // right, left, up, down

    // To the right =>
    for j in (pos_x+1)..map[0].len() {
        score[0] += 1;
        if map[pos_y][j] >= map[pos_y][pos_x] {
            break;
        }
    }

    // To the left  <=
    for j in (0..(pos_x)).rev() {
        score[1] += 1;
        if map[pos_y][j] >= map[pos_y][pos_x] {
            break;
        }
    }

    // To the bottom v
    for i in (pos_y+1)..map.len() {
        score[3] += 1;
        if map[i][pos_x] >= map[pos_y][pos_x] {
            break;
        }
    }

    // To the top ^
    for i in (0..pos_y).rev() {
        score[2] += 1;
        if map[i][pos_x] >= map[pos_y][pos_x] {
            break;
        }
    }

    let mut res: u32 = 1;
    score.iter().for_each(|x| res *= x);
    res
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
    let mut scenic_map  = create_empty_map(map.len(), map[0].len());

    // Iterate the map and get the scenic score for each tree
   for i in 0..map.len(){
       for j in 0..map[0].len(){
            scenic_map[i][j] = get_scenic_score(i, j, &map);
       }
   }

    println!("\nFinal Scenic Map:");
    print_map(&scenic_map);

    // Get the maximum value
    let mut max = 0;
    for i in 0..scenic_map.len() {
        let max_line = scenic_map[i].iter().max().unwrap();
        max = if max_line > &max {*max_line} else {max};
    }
    println!("Tree with the most visiblity score: {}", max);

    Ok(())
}

