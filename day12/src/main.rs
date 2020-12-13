use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
enum Orientation {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Boat {
    x: i64,
    y: i64,
    heading: Orientation,
}

#[derive(Debug)]
struct Waypoint {
    x: i64,
    y: i64,
}

impl Waypoint {
    fn trudge(&mut self, instruction: &str) -> &Waypoint {
        let val: i64 = instruction[1..].parse::<i64>().unwrap();
        if let Some(c) = instruction.chars().next() {
            match c {
                'N' => self.y += val,
                'S' => self.y -= val,
                'E' => self.x += val,
                'W' => self.x -= val,
                'R' => match val {
                    90 => {
                        let t = self.x;
                        self.x = self.y;
                        self.y = -t;
                    }
                    180 => {
                        self.x = -self.x;
                        self.y = -self.y;
                    }
                    270 => {
                        let t = self.x;
                        self.x = -self.y;
                        self.y = t;
                    }
                    _ => {}
                },
                'L' => match val {
                    90 => {
                        let t = self.x;
                        self.x = -self.y;
                        self.y = t;
                    }
                    180 => {
                        self.x = -self.x;
                        self.y = -self.y;
                    }
                    270 => {
                        let t = self.x;
                        self.x = self.y;
                        self.y = -t;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        self
    }
}

impl Boat {
    fn trudge_2(&mut self, waypoint: &Waypoint, instruction: &str) -> &Boat {
        let val: i64 = instruction[1..].parse::<i64>().unwrap();
        if let Some(c) = instruction.chars().next() {
            if let 'F' = c {
                self.x += val * waypoint.x;
                self.y += val * waypoint.y;
            }
        }
        self
    }

    fn trudge(&mut self, instruction: &str) -> &Boat {
        let val: i64 = instruction[1..].parse::<i64>().unwrap();
        if let Some(c) = instruction.chars().next() {
            match c {
                'N' => self.y += val,
                'S' => self.y -= val,
                'E' => self.x += val,
                'W' => self.x -= val,
                'F' => match &self.heading {
                    Orientation::North => self.y += val,
                    Orientation::South => self.y -= val,
                    Orientation::East => self.x += val,
                    Orientation::West => self.x -= val,
                },
                'L' => match &self.heading {
                    Orientation::North => match val {
                        90 => self.heading = Orientation::West,
                        180 => self.heading = Orientation::South,
                        270 => self.heading = Orientation::East,
                        _ => {}
                    },
                    Orientation::South => match val {
                        90 => self.heading = Orientation::East,
                        180 => self.heading = Orientation::North,
                        270 => self.heading = Orientation::West,
                        _ => {}
                    },
                    Orientation::East => match val {
                        90 => self.heading = Orientation::North,
                        180 => self.heading = Orientation::West,
                        270 => self.heading = Orientation::South,
                        _ => {}
                    },
                    Orientation::West => match val {
                        90 => self.heading = Orientation::South,
                        180 => self.heading = Orientation::East,
                        270 => self.heading = Orientation::North,
                        _ => {}
                    },
                },
                'R' => match &self.heading {
                    Orientation::North => match val {
                        90 => self.heading = Orientation::East,
                        180 => self.heading = Orientation::South,
                        270 => self.heading = Orientation::West,
                        _ => {}
                    },
                    Orientation::South => match val {
                        90 => self.heading = Orientation::West,
                        180 => self.heading = Orientation::North,
                        270 => self.heading = Orientation::East,
                        _ => {}
                    },
                    Orientation::East => match val {
                        90 => self.heading = Orientation::South,
                        180 => self.heading = Orientation::West,
                        270 => self.heading = Orientation::North,
                        _ => {}
                    },
                    Orientation::West => match val {
                        90 => self.heading = Orientation::North,
                        180 => self.heading = Orientation::East,
                        270 => self.heading = Orientation::South,
                        _ => {}
                    },
                },
                _ => {}
            }
        }
        self
    }
}

fn main() -> io::Result<()> {
    // read input
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut boat: Boat = Boat {
        x: 0,
        y: 0,
        heading: Orientation::East,
    };
    let mut boat_2: Boat = Boat {
        x: 0,
        y: 0,
        heading: Orientation::East,
    };
    let mut waypoint: Waypoint = Waypoint { x: 10, y: 1 };
    for line_r in reader.lines() {
        if let Ok(line) = line_r {
            boat.trudge(&line);
            waypoint.trudge(&line);
            boat_2.trudge_2(&waypoint, &line);
        }
    }
    println!("Day 1: {}", boat.x.abs() + boat.y.abs());
    println!("Day 2: {}", boat_2.x.abs() + boat_2.y.abs());

    Ok(())
}
