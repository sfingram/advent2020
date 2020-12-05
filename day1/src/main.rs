use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

fn main() -> io::Result<()> {

    // read input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // parse lines
    let values: Vec<i32> = reader.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    // iterate over all pairs of values
    let (a, b) = values[0..values.len()-1].iter()
        .enumerate()
        .map(|(i, x)| values[i+1..values.len()].iter()
                        .map(|y| (x, y))
                        .find(|z| z.0+z.1 == 2020))
        .filter(|v| matches!(v, Some((_, _))))
        .map(|v| v.unwrap())
        .next().unwrap();
    println!("Part 1: {} + {} == 2020, a * b == {}", a, b, a*b);

    // iterate over all triples of values
    let (a, b, c) = values[0..values.len()-2].iter()
        .enumerate()
        .map(|(i, x)| values[i+1..values.len()-1].iter()
            .enumerate()
            .map(|(j, y)| values[i+j+1..values.len()].iter()
                .map(|z| (x, y, z))
                .find(|z| z.0+z.1+z.2 == 2020)
            )
            .filter(|v| matches!(v, Some((_, _, _))))
            .map(|v| v.unwrap())
            .next()
        )
        .filter(|v| matches!(v, Some((_, _, _))))
        .map(|v| v.unwrap())
        .next().unwrap();
    println!("Part 2: {} + {} + {} == 2020, a * b * c == {}", a, b, c, a*b*c);
    Ok(())
}
