use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    // read input
    let args: Vec<String> = env::args().collect();
    let preamble = args[1].parse::<i64>().unwrap();
    let filename = &args[2];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let values: Vec<i64> = reader
        .lines()
        .map(|v| v.unwrap().parse::<i64>().unwrap())
        .collect();

    let mut sum_checker: HashMap<i64, i32> = HashMap::new();

    // initialize the sum checker
    for i in 0..preamble {
        for j in 0..preamble {
            // skip self-sums
            if i > j {
                let x = values[i as usize] + values[j as usize];
                let s = match sum_checker.get(&x) {
                    Some(x) => *x,
                    _ => 0,
                };
                sum_checker.insert(x, s + 1);
            }
        }
    }

    let mut last_i: usize = 0;
    for i in (preamble as usize)..values.len() {
        last_i = i;

        // check if the value is a possible sum
        if !sum_checker.contains_key(&values[i]) {
            break;
        }

        // remove outgoing entries in the sum structures
        let del_i = i - (preamble as usize);
        for j in 1..(preamble) {
            if (del_i + (j as usize)) < values.len() {
                let s = values[del_i] + values[del_i + (j as usize)];
                let new_entry: i32 = *sum_checker.get(&s).unwrap() - 1;
                if new_entry > 0 {
                    sum_checker.insert(s, new_entry);
                } else {
                    sum_checker.remove(&s);
                }
            }
        }

        // add incoming entries to the sum structures
        for j in 1..(preamble) {
            let s = values[i] + values[i - (j as usize)];
            let new_entry: i32 = match sum_checker.get(&s) {
                Some(x) => *x + 1,
                _ => 1,
            };
            sum_checker.insert(s, new_entry);
        }
    }
    println!("\nPart 1: {}", values[last_i]);

    let mut start = 0;
    let mut end = 0;
    let mut runsum = values[0];
    while runsum != values[last_i] {
        if runsum < values[last_i] {
            end += 1;
            runsum += values[end];
        } else {
            runsum -= values[start];
            start += 1;
        }
    }
    println!(
        "Part 2: {}",
        values[start..end + 1].iter().max().unwrap() + values[start..end + 1].iter().min().unwrap()
    );

    Ok(())
}
