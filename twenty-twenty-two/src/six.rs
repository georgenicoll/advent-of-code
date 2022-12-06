use std::io::Error;
use std::collections::{VecDeque, HashMap};
use std::convert::identity;
use crate::utils;

const FILENAME: &str = "6/input.txt";

pub fn _6a() -> Result<usize, Error> {
    run(|info, line| accumulate(info, line, 4))
}

pub fn _6b() -> Result<usize, Error> {
    run(|info, line| accumulate(info, line, 14))
}

fn run(accumulate_fn: fn (Info, String) -> Info) -> Result<usize, Error> {
    utils::process_file(
        FILENAME,
        identity,
        Info::new(),
        accumulate_fn,
        reduce
    )
}

#[derive(Debug)]
struct Info {
    pos: usize
}

impl Info {
    pub fn new() -> Info {
        Info {
            pos: 0,
        }
    }
}

fn accumulate(mut info: Info, line: String, len_to_detect: usize) -> Info {
    let mut last_n: VecDeque<char> = VecDeque::with_capacity(len_to_detect);
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut num_distinct_letters = 0;
    for (pos, c) in line.chars().enumerate() {
        if last_n.len() == len_to_detect {
            let front = last_n.pop_front().unwrap();
            let count = map.entry(front).and_modify(|count| *count -= 1).or_insert(0);
            if *count == 0 {
                num_distinct_letters -= 1;
            }
        }
        let count = map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        if *count == 1 {
            num_distinct_letters += 1;
            if num_distinct_letters >= len_to_detect {
                info.pos = pos;
                break;
            }
        }
        last_n.push_back(c);
    }
    info
}

fn reduce(info: Info) -> usize {
    info.pos + 1
}