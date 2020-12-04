use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashSet;
#[macro_use] extern crate lazy_static;

// static features: HashSet<String> = ["byr".to_string()].iter().cloned().collect( );

lazy_static! {
    static ref REF_FEATURES: HashSet<String> = [
        "byr", 
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ].iter().map(|x| x.to_string()).collect( );
}

fn validate(passport: &Vec<String>) -> bool {
    let features: HashSet<String> = passport.iter().map(|x| x.split(':').next().unwrap().to_string()).collect();
    return REF_FEATURES.is_subset(&features)
}

fn main() -> io::Result<()> {
    // read input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // parse and count trees
    let mut entries: Vec<Vec<String>> = Vec::new();
    entries.push(Vec::new());

    let values: Vec<String> = reader.lines().map(|v| v.unwrap()).collect();
    entries = values.iter().fold(entries, |mut acc, x| {
        if x.len() == 0 {
            acc.push(Vec::new());
        } else {
            let snarf = acc.pop().unwrap();
            acc.push(x.split_ascii_whitespace().fold(snarf, |mut inner_acc, y| {
                inner_acc.push(y.to_string());
                inner_acc
            }));
        }
        acc
    });

    println!("Part 1: {}", entries.iter().filter(|x| validate(&x)).count());

    return Ok(());
}
