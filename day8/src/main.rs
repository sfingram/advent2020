use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
// use std::collections::{HashMap, HashSet};
#[macro_use]
extern crate lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?P<opcode>[[:alpha:]]+) (?P<sign>[+\-])(?P<arg>\d+)$").unwrap();
}

struct Op<'a> {
    opcode: &'a str,
    arg: i32,
    read: bool,
}

fn main() -> io::Result<()> {
    // read input

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let values: Vec<String> = reader.lines().map(|v| v.unwrap()).collect();

    let mut ops: Vec<Op> = values
        .iter()
        .map(|value| {
            let value_captures = RE.captures(&value).unwrap();
            let sign: i32 = if value_captures.name("sign").unwrap().as_str() == "-" {
                -1
            } else {
                1
            };
            Op {
                opcode: value_captures.name("opcode").unwrap().as_str(),
                arg: sign
                    * value_captures
                        .name("arg")
                        .unwrap()
                        .as_str()
                        .parse::<i32>()
                        .unwrap(),
                read: false,
            }
        })
        .collect();

    let mut done: bool = false;
    let mut pc: i32 = 0;
    let mut acc: i32 = 0;
    while !done {
        if !ops[pc as usize].read {
            ops[pc as usize].read = true;
            match ops[pc as usize].opcode {
                "acc" => {
                    acc += ops[pc as usize].arg;
                    pc += 1;
                }
                "jmp" => pc += ops[pc as usize].arg,
                "nop" => pc += 1,
                _ => (),
            }
        } else {
            done = true;
        }
    }

    println!("Part 1: {}", acc);

    let mut stack: Vec<(i32, i32, usize)> = Vec::new();
    let mut op_stack: Vec<usize> = Vec::new();
    let mut op_stack_size: usize;

    // reset state
    ops.iter_mut().for_each(|x| x.read = false);
    let ops_len = ops.len();
    pc = 0;
    acc = 0;

    while (pc as usize) < ops_len {
        if !ops[pc as usize].read {
            ops[pc as usize].read = true;
            op_stack.push(pc as usize);
            match ops[pc as usize].opcode {
                "acc" => {
                    acc += ops[pc as usize].arg;
                    pc += 1;
                }
                "jmp" => {
                    if !stack.is_empty() {
                        pc += ops[pc as usize].arg;
                    } else {
                        stack.push((pc, acc, op_stack.len()));
                        pc += 1
                    }
                }
                "nop" => {
                    if !stack.is_empty() {
                        pc += 1
                    } else {
                        stack.push((pc, acc, op_stack.len()));
                        pc += ops[pc as usize].arg;
                    }
                }
                _ => (),
            }
        } else {
            // pop the state
            let triple = stack.pop().unwrap();
            pc = triple.0;
            acc = triple.1;
            op_stack_size = triple.2;

            // rewind the op_stack
            while op_stack.len() > op_stack_size {
                let pc_rw = op_stack.pop().unwrap();
                ops[pc_rw].read = false;
            }

            // perform the op as normal
            match ops[pc as usize].opcode {
                "jmp" => {
                    pc += ops[pc as usize].arg;
                }
                "nop" => pc += 1,
                _ => (),
            }
        }
    }
    println!("Part 2: {}", acc);

    Ok(())
}
