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
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut original: Vec<char> = Vec::new();
    for line_r in reader.lines() {
        if let Ok(line) = line_r {
            height += 1;
            width = line.len();
            line.chars().for_each(|c| original.push(c));
        }
    }
    let mut values: Vec<char> = original.clone();
    let mut values_copy: Vec<char> = values.clone();

    let mut changed = true;
    while changed {
        changed = false;
        for j in 0..height {
            for i in 0..width {
                let current_state: char = values[j * width + i];
                let next_state: char = match current_state {
                    'L' => {
                        // If a seat is empty (L) and there are no occupied
                        // seats adjacent to it, the seat becomes occupied
                        let mut found_occupied: bool = false;
                        for h in -1 as i64..2 {
                            for v in -1 as i64..2 {
                                let x_pos = h + (i as i64);
                                let y_pos = v + (j as i64);
                                if !(h == 0 && v == 0)
                                    && (x_pos >= 0
                                        && x_pos < width as i64
                                        && y_pos >= 0
                                        && y_pos < height as i64)
                                    && values[(y_pos * width as i64 + x_pos) as usize] == '#'
                                {
                                    found_occupied = true;
                                }
                            }
                        }
                        if !found_occupied {
                            changed = true;
                            '#'
                        } else {
                            current_state
                        }
                    }
                    '#' => {
                        // If a seat is occupied (#) and four or more seats
                        // adjacent to it are also occupied, the seat becomes empty
                        let mut occupied_count: i64 = 0;
                        for h in -1 as i64..2 {
                            for v in -1 as i64..2 {
                                let x_pos = h + (i as i64);
                                let y_pos = v + (j as i64);
                                if !(h == 0 && v == 0)
                                    && x_pos >= 0
                                    && x_pos < width as i64
                                    && y_pos >= 0
                                    && y_pos < height as i64
                                    && values[(y_pos * width as i64 + x_pos) as usize] == '#'
                                {
                                    occupied_count += 1;
                                }
                            }
                        }
                        if occupied_count >= 4 {
                            changed = true;
                            'L'
                        } else {
                            current_state
                        }
                    }
                    // Otherwise, the seat's state does not change
                    _ => current_state,
                };
                values_copy[j * width + i] = next_state;
            }
        }

        values_copy
            .iter()
            .enumerate()
            .for_each(|(i, c)| values[i] = *c);
    }

    // How many seats end up occupied?
    println!("Part 1: {}", values.iter().filter(|x| **x == '#').count());

    values = original;
    let lines_of_sight: Vec<(i64, i64)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut changed = true;
    while changed {
        changed = false;
        for j in 0..height {
            for i in 0..width {
                let current_state: char = values[j * width + i];
                let next_state: char = match current_state {
                    'L' => {
                        // If a seat is empty (L) and there are no occupied
                        // seats adjacent to it, the seat becomes occupied
                        let mut found_occupied: bool = false;
                        for (los_x, los_y) in lines_of_sight.iter() {
                            let mut h = i as i64 + los_x;
                            let mut v = j as i64 + los_y;
                            while h >= 0 && v >= 0 && h < width as i64 && v < height as i64 {
                                if values[(v * width as i64 + h) as usize] != '.' {
                                    if values[(v * width as i64 + h) as usize] == '#' {
                                        found_occupied = true;
                                    }
                                    break;
                                }
                                h += los_x;
                                v += los_y;
                            }
                        }
                        if !found_occupied {
                            changed = true;
                            '#'
                        } else {
                            current_state
                        }
                    }
                    '#' => {
                        // If a seat is occupied (#) and four or more seats
                        // adjacent to it are also occupied, the seat becomes empty
                        let mut occupied_count: i64 = 0;
                        for (los_x, los_y) in lines_of_sight.iter() {
                            let mut h = i as i64 + los_x;
                            let mut v = j as i64 + los_y;
                            while h >= 0 && v >= 0 && h < width as i64 && v < height as i64 {
                                if values[(v * width as i64 + h) as usize] != '.' {
                                    if values[(v * width as i64 + h) as usize] == '#' {
                                        occupied_count += 1;
                                    }
                                    break;
                                }
                                h += los_x;
                                v += los_y;
                            }
                        }
                        if occupied_count >= 5 {
                            changed = true;
                            'L'
                        } else {
                            current_state
                        }
                    }
                    // Otherwise, the seat's state does not change
                    _ => current_state,
                };
                values_copy[j * width + i] = next_state;
            }
        }

        values_copy
            .iter()
            .enumerate()
            .for_each(|(i, c)| values[i] = *c);
    }

    // How many seats end up occupied?
    println!("Part 2: {}", values.iter().filter(|x| **x == '#').count());

    Ok(())
}
