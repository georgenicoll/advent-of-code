use std::collections::HashSet;
use std::fmt::Display;
use std::cmp;

use crate::utils;

const FILE_NAME: &str = "20/input.txt";
// const FILE_NAME: &str = "20/test_input.txt";
// const FILE_NAME: &str = "20/my_test_input.txt";

pub fn _20a() -> Result<i64, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate1, reduce1)
}

pub fn _20b() -> Result<i64, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate2, reduce2)
}

fn parse_line(line: String) -> i64 {
    line.parse().unwrap()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Number {
    number: i64,
    original_position: usize,
}

impl Number {
    pub fn new(number: i64, original_position: usize) -> Number {
        Number {
            number,
            original_position,
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.number, self.original_position)
    }
}

struct State {
    numbers: Vec<Number>,
}

impl State {
    pub fn new() -> State {
        State {
            numbers: Vec::new(),
        }
    }
}

fn accumulate1(mut state: State, number: i64) -> State {
    state.numbers.push(Number::new(number, state.numbers.len()));
    state
}

fn reduce1(mut state: State) -> i64 {
    let numbers = &mut state.numbers;

    process(numbers);

    calculate_final_value(numbers)
}

const MAGIC_NUMBER: i64 = 811589153;
const TIMES_TO_DECRYPT: usize = 10;

fn accumulate2(mut state: State, number: i64) -> State {
    state.numbers.push(Number::new(number * MAGIC_NUMBER, state.numbers.len()));
    state
}

fn reduce2(mut state: State) -> i64 {
    let original_numbers = state.numbers.clone();
    let numbers = &mut state.numbers;

    for iteration in 0..TIMES_TO_DECRYPT {
        println!("Processing iteration {}", iteration);
        process2(&original_numbers, numbers);
    }

    calculate_final_value(numbers)
}

fn calculate_final_value(numbers: &Vec<Number>) -> i64 {
    let zero_index = find_zero(numbers).unwrap();

    let index_1000 = (zero_index + 1000) % numbers.len();
    let index_2000 = (zero_index + 2000) % numbers.len();
    let index_3000 = (zero_index + 3000) % numbers.len();

    let value_1000 = numbers.get(index_1000).unwrap().number;
    let value_2000 = numbers.get(index_2000).unwrap().number;
    let value_3000 = numbers.get(index_3000).unwrap().number;

    println!("{} + {} + {}", value_1000, value_2000, value_3000);
    value_1000 + value_2000 + value_3000
}

fn process(numbers: &mut Vec<Number>) {
    let mut already_processed: HashSet<Number> = HashSet::new();
    let mut num_processed = 0;
    let mut current_index: usize = 0;
    let len = numbers.len();

    while num_processed < numbers.len() && current_index < numbers.len() {
        let num = {
            let number = numbers.get(current_index).unwrap();
            if already_processed.contains(&number) {
                current_index += 1;
                continue;
            }
            already_processed.insert(number.clone());
            number.number
        };
        shift_number_at(numbers, &mut current_index, len, num);
        num_processed += 1;
    }
}

fn process2(original_numbers: &Vec<Number>, numbers: &mut Vec<Number>) {
    let len = numbers.len();

    for number in original_numbers {
        let mut current_index = find_index_of(number, numbers);

        shift_number_at(numbers, &mut current_index, len, number.number);
    }
}

fn shift_number_at(numbers: &mut Vec<Number>, current_index: &mut usize, len: usize, num: i64) {
    let new_index = calculate_new_index(*current_index, len, num);
    if new_index != *current_index {
        let number = numbers.remove(*current_index);
        if new_index < *current_index {
            numbers.insert(new_index, number);
            *current_index += 1
        } else {
            numbers.insert(new_index, number);
        }
    } else {
        *current_index += 1;
    }
}

fn calculate_new_index(current_index: usize, len: usize, num: i64) -> usize {
    if num == 0 {
        return current_index;
    }

    let abs_num = num.abs();
    let max_index = len - 1;
    let abs_num_modded = abs_num % max_index as i64;
    let mut still_to_move = abs_num_modded;

    if num < 0 {
        let mut pos = current_index;
        while still_to_move > 0 {
            if pos == 0 {
                pos = max_index;
            }
            let this_move = cmp::min(pos as i64, still_to_move);
            still_to_move -= this_move;
            pos -= this_move as usize;
        }
        return pos;
    }

    let mut pos = current_index;
    while still_to_move > 0 {
        if pos == max_index {
            pos = 0;
        }
        let this_move = cmp::min((max_index - pos) as i64, still_to_move);
        still_to_move -= this_move;
        pos += this_move as usize;
    }
    pos
}

fn find_zero(numbers: &Vec<Number>) -> Option<usize> {
    for (index, value) in numbers.iter().enumerate() {
        if value.number == 0 {
            return Some(index);
        }
    }
    None
}

fn find_index_of(number_to_look_for: &Number, numbers: &Vec<Number>) -> usize {
    for (index, number) in numbers.iter().enumerate() {
        if number_to_look_for == number {
            return index;
        }
    }
    panic!("Didn't find {}", number_to_look_for);
}