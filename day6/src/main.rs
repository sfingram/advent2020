use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::{HashSet};

fn main() -> io::Result<()> {
    
    // read input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let values: Vec<String> = reader.lines().map(|v| v.unwrap()).collect();

    // group input into a vector of hash sets
    let mut groups: Vec<HashSet<String>> = Vec::new();
    groups.push(HashSet::new());
    let part_1: usize = values.iter()
        .fold(groups, |mut acc, x| {
            if x.is_empty() {
                acc.push(HashSet::new());
            } else {
                let snarf = acc.pop().unwrap();
                acc.push(x.chars().fold(snarf, |mut inner_acc, y| {
                    inner_acc.insert(y.to_string());
                    inner_acc
                }));
            }
            acc
        }).iter()
        .map(|x| x.len())
        .sum();
    
    println!("Part 1: {}", part_1);

    // group chars into a vector of vector of sets
    let mut intersections: Vec<Vec<HashSet<char>>> = Vec::new();
    intersections.push(Vec::new());
    let part_2: usize = values.iter()
        .fold(intersections, |mut acc, x| {
            if x.is_empty() {
                acc.push(Vec::new());
            } else {
                let mut sets = acc.pop().unwrap();
                let mut set : HashSet<char> = HashSet::new();
                x.chars().for_each(|y| {set.insert(y);});
                sets.push(set);
                acc.push(sets);
            }
            acc
        })
        .iter()
        // get the len of the intersection of all the sets in each vector entry
        .map(|x| {
            x.iter()
                .fold(x[0].clone(), |intersection, i| 
                    intersection.intersection(i)
                        .copied()
                        .collect::<HashSet<char>>()
                )
                .len()
        })
        .sum();

    println!("Part 2: {}", part_2);

    Ok(())
}
