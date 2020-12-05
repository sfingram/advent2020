use itertools::{sorted, zip};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    // read input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let ids: Vec<i32> = reader
        .lines()
        .map(|v| {
            let v_u = v.unwrap();
            let row: i32 = v_u[..7].to_string()
            .chars()
            .enumerate()
            .map(|(i, x)| match x {
                'B' => 1,
                _ => 0,
            } << (6-i))
            .sum();
            let col: i32 = v_u[7..].to_string()
            .chars()
            .enumerate()
            .map(|(i, x)| match x {
                'R' => 1,
                _ => 0,
            } << (2-i))
            .sum();
            row * 8 + col
        })
        .collect();

    let part_1: i32 = *ids.iter().max().unwrap();

    println!("Part 1: {}", part_1);

    let sorted_ids: Vec<i32> = sorted(ids.iter()).copied().collect();
    let (part_2, _) = zip(sorted_ids.iter(), sorted_ids[1..].iter())
        .find(|(x, y)| *x + 1 != **y)
        .unwrap();

    println!("Part 2: {}", part_2 + 1);

    Ok(())
}
