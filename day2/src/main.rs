use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::str::FromStr;
use regex::Regex;
#[macro_use] extern crate lazy_static;

struct Password {
    minimum: i32,
    maximum: i32,
    letter: char,
    value: String,
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}) - {} : {}", self.minimum, self.maximum, self.letter, self.value)
    }
}

impl Password {
    fn is_valid(&self) -> bool {
        let v = self.value.chars().filter(|x| x == &self.letter).count();
        v >= self.minimum as usize && v <= self.maximum as usize
    }

    fn is_valid_too(&self) -> bool {
        let first = self.value.chars().nth((self.minimum-1) as usize).unwrap_or('0') == self.letter;
        let second =  self.value.chars().nth((self.maximum-1) as usize).unwrap_or('0') == self.letter;
        (first && !second) || (!first && second)
    }
}

impl FromStr for Password {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+)+ ([[:alpha:]]): ([[:alpha:]]+)").unwrap();
        }
        let cap = RE.captures(input).unwrap();
        Ok(Password {
            minimum: cap[1].parse::<i32>().unwrap(),
            maximum: cap[2].parse::<i32>().unwrap(),
            letter: cap[3].as_bytes()[0] as char,
            value: cap[4].to_string(),
        })
    }
}

fn main() -> io::Result<()> {

    // read input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // parse and count valid entries
    let values: Vec<Password> = reader.lines()
        .map(|v| Password::from_str(&v.unwrap()).unwrap())
        .collect();
    
    let num_values = values.iter()
        .filter(|v| v.is_valid())
        .count();

    println!("Part 1: {}", num_values);

    let num_values_too = values.iter()
        .filter(|v| v.is_valid_too())
        .count();

    println!("Part 2: {}", num_values_too);

    Ok(())
}
