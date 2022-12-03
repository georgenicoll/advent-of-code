use crate::utils;
use std::io::Error;
use std::collections::HashSet;
use std::convert::identity;


pub fn _3a() -> Result<u64, Error> {
    let filename = "3/input.txt";
    utils::process_file(
        filename,
        parse_line1,
        0,
        accumulate1,
        identity,
    )
}

pub fn _3b() -> Result<u64, Error> {
    let filename = "3/input.txt";
    utils::process_file(
        filename,
        identity,
        GroupDetails::new(HashSet::new(), 0, 0),
        accumulate2,
        reduce2,
    )
}

fn parse_line1(line: String) -> Option<char> {
    //Assume bytes are letters
    let first_half = &line[0..line.len() / 2];
    let second_half = &line[line.len()/2..];
    let collector: fn (HashSet<char>, char) -> HashSet<char> = |mut set, c| {
        set.insert(c);
        set
    };
    let first_items: HashSet<char> = first_half.chars().fold(HashSet::new(), collector);
    for (_, c) in second_half.chars().enumerate() {
        if first_items.contains(&c) {
            //we found it... can exit now
            return Some(c);
        }
    }
    None
}

fn accumulate1(sum: u64, item_opt: Option<char>) -> u64 {
    match item_opt {
        Some(item) => sum + get_item_value(item),
        None => sum,
    }
}

fn get_item_value(item: char) -> u64 {
    match item {
        'A'..='Z' => ((item as u32 - 'A' as u32) + 27) as u64,
        'a'..='z' => ((item as u32 - 'a' as u32) + 1) as u64,
        _ => panic!("Unrecognised item: {}", item),
    }
}

struct GroupDetails {
    candidate_items: HashSet<char>,
    num: u8,
    sum: u64,
}

impl GroupDetails {
    pub fn new(candidate_items: HashSet<char>, num: u8, sum: u64) -> GroupDetails {
        GroupDetails {
            candidate_items,
            num,
            sum,
        }
    }
}

fn accumulate2(group_details: GroupDetails, line: String) -> GroupDetails {
    match group_details.num {
        0 => {
            //First of the group just put all of the chars into a candidate set
            GroupDetails::new(HashSet::from_iter(line.chars()), 1, group_details.sum)
        },
        1 => {
            //second line - candidate set should be the intersection of the candidates and these chars so keep if we find in both
            let mut new_candidates = HashSet::new();
            for (_, c) in line.chars().enumerate() {
                if group_details.candidate_items.contains(&c) {
                    new_candidates.insert(c);
                }
            }
            GroupDetails::new(new_candidates, 2, group_details.sum)
        },
        2 => {
            //third line - try to find the one that is matching - we'll assume there is going to be only 1
            for (_, c) in line.chars().enumerate() {
                if group_details.candidate_items.contains(&c) {
                    let item_priority = get_item_value(c);
                    return GroupDetails::new(HashSet::new(), 0, group_details.sum + item_priority)
                }
            }
            panic!("Didn't find an item common to all 3 lines - third group line is '{}'", line)
        },
        _ => panic!("Reached invalid GroupDetails num: {}", group_details.num)
    }
}

fn reduce2(group_details: GroupDetails) -> u64 {
    group_details.sum
}