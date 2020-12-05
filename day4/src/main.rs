use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::{HashSet};
#[macro_use] extern crate lazy_static;
use regex::Regex;


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
    static ref HGT_CM_RE: Regex = Regex::new(r"^(\d+)cm$").unwrap();
    static ref HGT_IN_RE: Regex = Regex::new(r"^(\d+)in$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(?:(?:amb)|(?:blu)|(?:brn)|(?:gry)|(?:grn)|(?:hzl)|(?:oth))$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    
}

fn validate(passport: &[String]) -> bool {
    let features: HashSet<String> = passport.iter()
        .map(|x| x.split(':')
            .next()
            .unwrap()
            .to_string())
        .collect();
    REF_FEATURES.is_subset(&features)
}

fn validate_more(passport: &[String]) -> bool {
    
    fn validate_dates(value: &str, start: i32, end: i32) -> bool {
        let mut check = false;
        if value.len() == 4 {
            let date = value.parse::<i32>().unwrap();
            check = date >= start && date <= end;   
        }
        check
    }

    passport.iter()
        .map(|x| {
            let mut split_fields = x.split(':');
            let (field, value) = (
                split_fields.next().unwrap(), 
                split_fields.next().unwrap()
            );
            match field {
                "byr" => validate_dates(&value, 1920, 2002),
                "iyr" => validate_dates(&value, 2010, 2020),
                "eyr" => validate_dates(&value, 2020, 2030),
                "hgt" => {
                    let mut check = false;
                    if HGT_CM_RE.is_match(&value) {
                        let height = HGT_CM_RE.captures(&value).unwrap()[1].parse::<i32>().unwrap();
                        check = height >= 150 && height <= 193;
                    }
                    if HGT_IN_RE.is_match(&value) {
                        let height = HGT_IN_RE.captures(&value).unwrap()[1].parse::<i32>().unwrap();
                        check = height >= 59 && height <= 76;
                    }
                    check
                },
                "hcl" => HCL_RE.is_match(&value),
                "ecl" => ECL_RE.is_match(&value),
                "pid" => PID_RE.is_match(&value),
                "cid" => true,
                _ => false
            }
        })
        .all(|x| x)
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
        if x.is_empty() {
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
    println!("Part 2: {}", passports.iter().filter(|x| validate(&x) && validate_more(&x)).count());

    Ok(())
}
