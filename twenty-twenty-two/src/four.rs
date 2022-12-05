use lazy_static::lazy_static;
use regex::Regex;

use crate::utils;
use std::{io::Error, convert::identity};


pub fn _4a() -> Result<u64, Error> {
    let filename = "4/input.txt";
    utils::process_file(
        filename,
        parse_line,
        0,
        accumulate1,
        identity,
    )
}

pub fn _4b() -> Result<u64, Error> {
    let filename = "4/input.txt";
    utils::process_file(
        filename,
        parse_line,
        0,
        accumulate2,
        identity,
    )
}

#[derive(Debug)]
struct Range {
  start: u64,
  end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Range {
        Range { start, end, }
    }
}

fn parse_line(line: String) -> (Range, Range) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    }
    match RE.captures(&line) {
        Some(caps) => {
            (
                Range::new(
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                ),
                Range::new(
                    caps.get(3).unwrap().as_str().parse().unwrap(),
                    caps.get(4).unwrap().as_str().parse().unwrap(),
                ),
            )
        },
        None => panic!("Line didn't match expected: {}", line),
    }
}

fn accumulate1(sum: u64, ranges: (Range, Range)) -> u64 {
    let (range1, range2) = ranges;
    if range_contains_other(&range1, &range2) || range_contains_other(&range2, &range1) {
        sum + 1
    } else {
        sum
    }
}

fn range_contains_other(range1: &Range, range2: &Range) -> bool {
    range1.start <= range2.start && range1.end >= range2.end
}

fn accumulate2(sum: u64, ranges: (Range, Range)) -> u64 {
    let (range1, range2) = ranges;
    if range_overlaps_other(&range1, &range2) || range_overlaps_other(&range2, &range1) {
        sum + 1
    } else {
        sum
    }
}

fn range_overlaps_other(range1: &Range, range2: &Range) -> bool {
    range1.start <= range2.end && range1.end >= range2.start
}
