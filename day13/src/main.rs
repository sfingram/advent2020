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
    let mut timestamp: i64 = 0;
    for (line_no, line) in reader.lines().enumerate() {
        match line_no {
            0 => {
                timestamp = line.unwrap().parse::<i64>().unwrap();
            },
            1 => {
                let (bus_no, wait_time) = line.unwrap()
                    .split(",")
                    .filter(|x| *x != "x")
                    .map(|x| x.parse::<i64>().unwrap())
                    .map(|x| (x, x - (timestamp % x)))
                    .min_by_key(|(_, y)| *y)
                    .unwrap();
                println!("Day 1: {}", bus_no * wait_time);
            },
            _ => ()
        }
    }
    // println!("Day 2: {}", boat_2.x.abs() + boat_2.y.abs());

    Ok(())
}
