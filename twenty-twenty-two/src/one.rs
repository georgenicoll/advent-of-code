use crate::utils;
use std::collections::BTreeSet;
use std::io::Error;

fn max(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn _1a() -> Result<i64, Error> {
    let filename = "1/input.txt";
    utils::process_file(
        filename,
        parse_line,
        (0, 0),
        accumulate,
        reduce,
    )
}

fn parse_line(line: String) -> Option<i64> {
    if line.trim().len() == 0 {
        None
    } else {
        let value: i64 = line.parse().unwrap();
        Some(value)
    }
}

fn accumulate(max_and_current: (i64, i64), next: Option<i64>) -> (i64, i64) {
    match next {
        Some(value) => (max_and_current.0, max_and_current.1 + value),
        None => (max(max_and_current.0, max_and_current.1), 0),
    }
}

fn reduce(max_and_current: (i64, i64)) -> i64 {
    max(max_and_current.0, max_and_current.1)
}

fn maybe_update_set(set: &mut BTreeSet<i64>, maybe_new_value: &mut i64) {
    set.insert(*maybe_new_value);
    if set.len() > 3 {
        set.pop_first();
    }
}

struct TopAndCurrent<'a> {
    top: &'a mut BTreeSet<i64>,
    current: &'a mut i64,
}

impl<'a> TopAndCurrent<'a> {
    pub fn new_boxed(top: &'a mut BTreeSet<i64>, current: &'a mut i64) -> Box<TopAndCurrent<'a>> {
        Box::new(Self {
            top,
            current,
        })
    }
}

pub fn _1b() -> Result<i64, Error> {
    let filename = "1/input.txt";
    utils::process_file(
        filename,
        parse_line,
        TopAndCurrent::new_boxed(&mut BTreeSet::new(), &mut 0),
        accumulator2,
        reduce2
    )


    // File::open(filename).map(BufReader::new).map(|reader| {
    //     let mut top3: BTreeSet<i64> = BTreeSet::new();
    //     let mut current_total: i64 = 0;
    //     for (_, line_res) in reader.lines().enumerate() {
    //         let line = line_res.unwrap();
    //         if line.trim().len() == 0 {
    //             maybe_update_set(&mut top3, current_total);
    //             current_total = 0
    //         } else {
    //             let parsed: i64 = line.parse().unwrap();
    //             current_total += parsed
    //         }
    //     }
    //     maybe_update_set(&mut top3, current_total);
    //     return top3.iter().fold(0, |acc, i| acc + i);
    // })
}

fn accumulator2(acc: Box<TopAndCurrent>, next: Option<i64>) -> Box<TopAndCurrent> {
    match next {
        Some(value) => {
            *acc.current = *acc.current + value;
            acc
        },
        None => {
            maybe_update_set(acc.top, acc.current);
            *acc.current = 0;
            acc
        },
    }
}

fn reduce2(acc: Box<TopAndCurrent>) -> i64 {
    maybe_update_set(acc.top, acc.current);
    acc.top.iter().fold(0, |acc, i| acc + i)
}
