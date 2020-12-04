use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashSet;
#[macro_use] extern crate lazy_static;

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
    let features: HashSet<String> = passport.iter()
        .map(|x| x.split(':')
            .next()
            .unwrap()
            .to_string())
        .collect();
    return REF_FEATURES.is_subset(&features)
}

fn main() -> io::Result<()> {
    // read input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut passports: Vec<Vec<String>> = Vec::new();
    passports.push(Vec::new());

    // group input into a vector of variable-length vectors
    let values: Vec<String> = reader.lines().map(|v| v.unwrap()).collect();
    passports = values.iter().fold(passports, |mut acc, x| {
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

    println!("Part 1: {}", passports.iter().filter(|x| validate(&x)).count());

    return Ok(());
}
