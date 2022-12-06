use lazy_static::lazy_static;
use regex::Regex;

use crate::utils;
use std::io::Error;
use std::collections::{HashMap, BTreeMap};


pub fn _5a() -> Result<String, Error> {
    let filename = "5/input.txt";
    utils::process_file(
        filename,
        parse_line,
        CrateStacks::new(),
        accumulate1,
        reduce,
    )
}

pub fn _5b() -> Result<String, Error> {
    let filename = "5/input.txt";
    utils::process_file(
        filename,
        parse_line,
        CrateStacks::new(),
        accumulate2,
        reduce,
    )
}

enum Line {
    Empty,
    Crates { crates_by_row: HashMap<usize, char>, },
    Rows { rows: Vec<usize>, },
    Move { num: usize, from: usize, to: usize, }
}

struct CrateStacks {
    stacks: BTreeMap<usize, Vec<char>>
}

impl CrateStacks {
    pub fn new() -> CrateStacks {
        CrateStacks {
            stacks: BTreeMap::new(),
        }
    }
}

fn parse_line(line: String) -> Line {
    match line.chars().next() {
        Some('m') => parse_move_line(&line),
        Some(' ') => parse_nums_line(&line),
        Some(_) => parse_crates_line(&line),
        None => Line::Empty,
    }
}

fn parse_move_line(line: &String) -> Line {
    lazy_static! {
        static ref MOVE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }
    let caps = MOVE.captures(line).unwrap();
    Line::Move {
        num: caps.get(1).unwrap().as_str().parse().unwrap(),
        from: caps.get(2).unwrap().as_str().parse().unwrap(),
        to: caps.get(3).unwrap().as_str().parse().unwrap(),
    }
}

fn parse_nums_line(line: &String) -> Line {
    let mut chars = line.chars();
    let mut rows: Vec<usize> = Vec::new();
    loop {
        let _space1 = chars.next();
        let row: Option<usize> = chars.next().and_then(|c| c.to_string().parse().ok());
        let _space2 = chars.next();
        match row {
            Some(row) => rows.push(row),
            None => break,
        }
        if chars.next().is_none() { //delimiter
            break;
        }
    }
    Line::Rows { rows }
}

fn parse_crates_line(line: &String) -> Line {
    let mut chars = line.chars();
    let mut crates: HashMap<usize, char> = HashMap::new();
    let mut row_index: usize = 0;
    loop {
        let _space1 = chars.next();
        let crate_id = chars.next();
        let _space2 = chars.next();
        match crate_id {
            Some(' ') => {
                row_index = row_index + 1
            },
            Some(id) => {
                crates.insert(row_index, id);
                row_index = row_index + 1;
            },
            None => break,
        }
        if chars.next().is_none() { //delimiter
            break;
        }
    }
    Line::Crates { crates_by_row: crates }
}

fn accumulate1(crate_stacks: CrateStacks, line: Line) -> CrateStacks {
    accumulate_gen(crate_stacks, line, move1)
}

fn move1(mut crate_stacks: CrateStacks, num: usize, from: usize, to: usize) -> CrateStacks {
    let mut items = {
        let from_stack = crate_stacks.stacks.get_mut(&from).unwrap();
        Vec::from_iter(from_stack.drain((from_stack.len() - num)..).rev())
    };
    crate_stacks.stacks.get_mut(&to).unwrap().  append(&mut items);
    crate_stacks
}

fn accumulate2(crate_stacks: CrateStacks, line: Line) -> CrateStacks {
    accumulate_gen(crate_stacks, line, move2)
}

fn move2(mut crate_stacks: CrateStacks, num: usize, from: usize, to: usize) -> CrateStacks {
    let mut items = {
        let from_stack = crate_stacks.stacks.get_mut(&from).unwrap();
        Vec::from_iter(from_stack.drain((from_stack.len() - num)..))
    };
    crate_stacks.stacks.get_mut(&to).unwrap().append(&mut items);
    crate_stacks
}

fn accumulate_gen(mut crate_stacks: CrateStacks,
                  line: Line,
                  move_func: fn (CrateStacks, usize, usize, usize) -> CrateStacks) -> CrateStacks {
    match line {
        Line::Empty => {
            for (_, stack) in crate_stacks.stacks.iter_mut() {
                stack.reverse()
            };
            crate_stacks
        },
        Line::Crates { crates_by_row } => {
            for (row, crate_id) in crates_by_row.iter() {
                match crate_stacks.stacks.get_mut(&row) {
                    Some(stack) => stack.push(*crate_id),
                    None => {
                        let mut new_stack = Vec::new();
                        new_stack.push(*crate_id);
                        crate_stacks.stacks.insert(*row, new_stack);
                    },
                };
            };
            crate_stacks
        },
        Line::Move { num, from, to } => move_func(crate_stacks, num, from, to),
        Line::Rows { rows } => {
            let mut new_stacks = CrateStacks::new();
            for (index, row) in rows.iter().enumerate() {
                let stack = crate_stacks.stacks.remove(&index).unwrap();
                new_stacks.stacks.insert(*row, stack);
            }
            new_stacks
        }
    }
}

fn reduce(crate_stacks: CrateStacks) -> String {
    let mut res = String::new();
    for (_, stack) in crate_stacks.stacks.iter() {
        match stack.last() {
            Some(c) => res.push(*c),
            None => res.push(' '),
        }
    };
    res
}

