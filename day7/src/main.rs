use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::{HashMap, HashSet};
#[macro_use] extern crate lazy_static;
use regex::Regex;

lazy_static! {
    static ref COLOR_RE: Regex = Regex::new(r"^(?P<color>[[:alpha:]]+ [[:alpha:]]+) bags contain (?P<contains>.+)[.]$").unwrap();
    static ref CONTAINS_RE: Regex = Regex::new(r"^(?P<quantity>(?:no)|(?:\d+)) (?P<color>(?:other)|([[:alpha:]]+ [[:alpha:]]+)) bag[s]?$").unwrap();
}

struct Bag<'a> {
    count: i32,
    color: &'a str,
    contain: Option<Vec<Bag<'a>>>,
}

impl Clone for Bag<'_> {
    fn clone(&self) -> Self {
        Bag{
            count: self.count,
            color: self.color,
            contain: self.contain.clone(),
        }
    }
}

impl Bag<'_> {

    fn new(x: &str) -> Bag {
        let color_caps = COLOR_RE.captures(x).unwrap();
        let contains_caps : Vec<Bag> = color_caps.name("contains")
            .unwrap()
            .as_str()
            .split(", ")
            .map(|y| {
                let contain_cap = CONTAINS_RE.captures(y).unwrap();
                match contain_cap.name("quantity").unwrap().as_str() {
                    "no" => None,
                    _ => {
                        Some(Bag{
                            count: contain_cap.name("quantity").unwrap().as_str().parse::<i32>().unwrap(),
                            color: contain_cap.name("color").unwrap().as_str(),
                            contain: None,
                        })
                    }
                }
            })
            .filter(|y| matches!(y, Some(_)))
            .map(|y| y.unwrap())
            .collect();
        Bag {
            count: 1,
            color: color_caps.name("color").unwrap().as_str(),
            contain: match contains_caps.len() {
                0 => None,
                _ => Some(contains_caps),
            },
        }
    }
}


fn main() -> io::Result<()> {
    
    // read input

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let values: Vec<String> = reader.lines().map(|v| v.unwrap()).collect();

    let mut bags : HashMap<&str, Bag> = HashMap::new();
    let mut inv_bags: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut part_1_bags : HashSet<&str> = HashSet::new();

    // loads the bags
    for value in values.iter() {
        let bag = Bag::new(value);
        bags.insert(bag.color, bag.clone());
    }

    // create an inverted index
    for key in bags.keys() {      
        if let Some(sub_bags) = &bags.get(key).unwrap().contain {
            for sub_bag in sub_bags.iter() {
                match inv_bags.get_mut(sub_bag.color) {
                    Some(inv_bags_entries) => {
                        let snarf = inv_bags_entries;
                        snarf.insert(key);
                    },
                    _ => {
                        let mut inv_bags_entries: HashSet<&str> = HashSet::new();
                        inv_bags_entries.insert(key);
                        inv_bags.insert(sub_bag.color, inv_bags_entries.clone());
                    }
                }
            }
        }  
    }

    // traverse the inverted index starting at shiny gold

    let mut cursor : HashSet<&str> = HashSet::new();
    cursor.insert("shiny gold");
    while ! cursor.is_empty() {
        let cursor_str = cursor.iter().cloned().next().unwrap();   
        if let Some(containing_bags) = inv_bags.get_mut(cursor_str) {
            for containing_bag in containing_bags.iter() {
                if ! part_1_bags.contains(containing_bag) {
                    cursor.insert(containing_bag);
                    part_1_bags.insert(containing_bag);
                }
            }
        }  
        cursor.remove(cursor_str);
    }
    println!("Part 1: {}", part_1_bags.len());

    // recursively traverse the regular index with counts

    fn delve(color: &str, baggies : &HashMap<&str, Bag>) -> i64 {
        match baggies.get(color) {
            Some(bag) => match &bag.contain {
                Some(contains) => contains
                    .iter()
                    .map(|x| x.count as i64 + x.count as i64 * delve(x.color, baggies))
                    .sum(),
                None => 0
            },
            None => 0,
        }
    }
    println!("Part 2: {}", delve("shiny gold", &bags));
    
    Ok(())
}
