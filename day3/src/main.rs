use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;


const TREE: char = '#';

struct Slope {
    horizontal: usize,
    vertical: usize,
}

fn check_slope(slope: &Slope, values: &[Vec<char>]) -> i32 {

    let mut h_pos: usize = 0;
    let mut num_trees: i32 = 0;    

    for (v_pos, value) in values.iter().enumerate() {
        if v_pos % slope.vertical == 0 {
            if value[h_pos % value.len()] == TREE {
                num_trees += 1;
            }
            h_pos += slope.horizontal;
        }
    }
    num_trees
}

fn main() -> io::Result<()> {

    // read input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // parse and count trees
    let values: Vec<Vec<char>> = reader.lines()
        .map(|v| v.unwrap().chars().collect())
        .collect();

    let slopes: [Slope; 5] = [
        Slope{horizontal: 1, vertical: 1}, 
        Slope{horizontal: 3, vertical: 1}, 
        Slope{horizontal: 5, vertical: 1}, 
        Slope{horizontal: 7, vertical: 1}, 
        Slope{horizontal: 1, vertical: 2}, 
    ];

    let num_trees_one = check_slope(&slopes[1], &values);
    println!("Part 1: {}", num_trees_one);

    let num_trees_two = slopes.iter()
        .map(|x| check_slope(x, &values))
        .fold(1 as i64, |acc, x| acc * (x as i64));
    println!("Part 2: {}", num_trees_two);

    Ok(())
}
