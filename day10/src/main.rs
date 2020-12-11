use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use itertools::{zip};
use itertools::Itertools;

fn main() -> io::Result<()> {

    // read input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut values: Vec<i64> = reader
        .lines()
        .map(|v| v.unwrap().parse::<i64>().unwrap())
        .collect();
    values.push(0);
    values.sort_unstable();
    values.push(values.last().unwrap()+3);

    let mut snarf : Vec<i64> = zip(values.iter(), values[1..].iter())
        .map(|(x, y)| y-x)
        .collect::<Vec<i64>>();
    let part_two : Vec<i64> = snarf.clone();
    snarf.sort_unstable();

    let mut ones = 0;
    let mut threes = 0;
    for (key, group) in &snarf.into_iter().group_by(|x| *x) {
        
        let g_len = group.count();
        match key {
            1 => ones = g_len,
            3 => threes = g_len,
            _ => ()
        }
    }

    println!("Part 1: {}", ones * threes);    
    
    // use a recurrence relation on lengths of 1 sequences
    fn f(k: i64) -> i64 {

        if k <= 2 {
            k
        } else if k == 3 {
            1 + f(1) + f(2)
        } else {
            f(k-1) + f(k-2) + f(k-3)
        }
    }

    let part_two_ans : i64 = part_two
        .iter()
        .group_by(|x| *x)
        .into_iter()        
        .map(|(key, group)| {
            let g_len = group.cloned().count();
            match key {
                1 => f(g_len as i64),
                3 => 1,
                _ => 1,
            }
        })
        .product();

    println!("Part 2: {}", part_two_ans);
    
    Ok(())
}
