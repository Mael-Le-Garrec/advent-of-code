mod entry;
use crate::entry::{Entry, EntryType};

#[cfg(test)]
#[macro_use]
extern crate log;

use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env::args;
use std::rc::Rc;
#[allow(unused_imports)]
use log::{debug, error, warn, log_enabled, info, Level};


fn create_tree(input: &str) -> Result<Rc<Entry>, std::io::Error>
{
    // Open the input file
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    // Create the root directory entry that will contain everything
    let mut tree = Entry::new("/", EntryType::Directory, 0);
    let mut current_entry = tree.borrow_mut();

    // Pointer for "find child" that needs to live long enough.
    // Idk how to make it better?
    let mut tmp_entry: Option<Rc<Entry>>;

    for l in reader.lines() {
        let line = l?;

        // Get the command
        let sp: Vec<&str> = line.split(" ").collect();
        match (sp[0], sp[1]) {
            ("$", "cd") => {  // Add the directory if it doesn't exist
                if sp[2] == "/" { // Go back to the root tree
                    current_entry = &mut tree;
                }
                else if sp[2] == ".." { // Go back to the parent
                    tmp_entry = current_entry.get_parent();
                    match tmp_entry.borrow_mut() {
                        Some(e) => {
                            info!("cd to parent directory {}", e.name);
                            current_entry = e;
                            }
                        None => { panic!("Can't cd to parent directory, doesn't exist.") }
                    }
                }
                else if !current_entry.contains_child(sp[2]) { // we can CD in a dir we haven't seen
                    let new_entry = Entry::new(sp[2], EntryType::Directory, 0);
                    current_entry.add_child(new_entry);
                    info!("cd into {}", sp[2]);
                }
                else {
                    info!("Change directory: {}", sp[2]);
                    tmp_entry = current_entry.find_child(sp[2]);
                    match tmp_entry.borrow_mut() {
                        Some(e) => {
                            info!("Found the entry, cd into {:?}", e);
                            current_entry = e;
                        },
                        None => { panic!("Can't cd, the entry does not exist") },
                    }
                }
            }
            ("$", "ls") => {
                // a ls doesn't do anything, we'll see in the next block
            }
            (&_, &_) => {  // Add the file or dir if it doesn't exist yet
                match sp[0] {
                    "dir" => {
                        if !current_entry.contains_child(sp[1]) {
                            let new_entry = Entry::new(sp[1], EntryType::Directory, 0);
                            current_entry.add_child(new_entry);
                            info!("Dir : {}", sp[1]);
                        }
                    }
                    &_ => {
                        let size: u32 = sp[0].parse().unwrap();
                        if !current_entry.contains_child(sp[1]) {
                            let new_entry = Entry::new(sp[1], EntryType::File, size);
                            current_entry.add_child(new_entry);
                            info!("File: {} with size {}", sp[1], size);
                        }
                    }
                }
            }
        }
    }
    return Ok(tree);
}

/// Returns the total size of directories having a size of at most `size`
fn get_total_size_dirs_atmost_size(entry: &Rc<Entry>, size: u32, sum_sizes: &mut u32) -> u32
{
    // If the entry is a file, return its size
    if entry.type_ == EntryType::File {
        return entry.size;
    }

    // Otherwise, iterate through the children
    let mut sum: u32 = 0;
    for child in entry.children.borrow().iter(){
        sum += get_total_size_dirs_atmost_size(child, size, sum_sizes);
    }

    if sum < size {
        *sum_sizes += sum;
        debug!("Directory {} has size {}", entry.name, sum);
    }

    return sum
}

/// Returns the smallest of directories having a size of at most `size`
fn get_smallest_dir_for_minimum_size(entry: &Rc<Entry>, size: u32, min_size: &mut u32) -> u32
{
    // If the entry is a file, return its size
    if entry.type_ == EntryType::File {
        return entry.size;
    }

    // Otherwise, iterate through the children
    let mut sum: u32 = 0;
    for child in entry.children.borrow().iter(){
        sum += get_smallest_dir_for_minimum_size(child, size, min_size);
    }

    if sum > size {
        debug!("Directory {} has size {}", entry.name, sum);
        if sum < *min_size {
            *min_size = sum;
            debug!("  It is currently the smallest one");
        }
    }

    return sum
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    // Read the arguments
    let input: &str;
    match args().nth(1) {
        Some(e) => input = if e == "--debug" {"input2"} else {"input"},
        None => input = "input",
    }

    // Get the tree from the given input
    let tree = create_tree(input)?;
    debug!("{:#?}", tree);
    println!("Size of tree: {}", tree.get_size());

    // Iterate on the tree to retrieve the directories with _at most_ a size of 100_000
    let max_size: u32 = 100_000;

    let mut sum_sizes = 0;
    let _sum = get_total_size_dirs_atmost_size(&tree, max_size, &mut sum_sizes);
    println!("Total size of directories with max size of {}: {}", max_size, sum_sizes);


    // Part 2
    let total_space = 70_000_000;
    let space_to_free = 30_000_000 - (total_space - tree.get_size());
    println!("Space to free: {}", space_to_free);
    let mut min_size: u32 = u32::MAX;
    let _ = get_smallest_dir_for_minimum_size(&tree, space_to_free, &mut min_size);
    println!("Smallest directory size that can be deleted: {}", min_size);

    Ok(())
}
