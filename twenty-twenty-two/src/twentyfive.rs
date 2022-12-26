use std::fmt::Display;
use std::collections::VecDeque;

use crate::utils;

const FILE_NAME: &str = "25/input.txt";
//const FILE_NAME: &str = "25/test_input.txt";
//const FILE_NAME: &str = "25/my_test_input.txt";

pub fn _25a() -> Result<String, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce)
}

pub fn _25b() -> Result<String, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce)
}

enum SnafuDigit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl Display for SnafuDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SnafuDigit::Two => "2",
            SnafuDigit::One => "1",
            SnafuDigit::Zero => "0",
            SnafuDigit::Minus => "-",
            SnafuDigit::DoubleMinus => "=",
        };
        write!(f, "{}", s)
    }
}

fn parse_line(line: String) -> Vec<SnafuDigit> {
    line.chars().into_iter()
        .map(|c| match c {
            '2' => SnafuDigit::Two,
            '1' => SnafuDigit::One,
            '0' => SnafuDigit::Zero,
            '-' => SnafuDigit::Minus,
            '=' => SnafuDigit::DoubleMinus,
            _ => panic!("Unrecognised snafu digit: {}", c),
        }).fold(
        Vec::with_capacity(line.len()),
        |mut digits, d| {
            digits.push(d);
            digits
        }
    )
}

struct Number {
    value: i128,
    snafu_digits: Vec<SnafuDigit>,
}

impl Number {
    pub fn new(value: i128, snafu_digits: Vec<SnafuDigit>) -> Number {
        Number { value, snafu_digits }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [", self.value)?;
        utils::output_into_iter(f, "", &mut self.snafu_digits.iter());
        write!(f, "]")
    }
}

struct State {
    numbers: Vec<Number>,
}

impl State {
    pub fn new() -> State {
        State { numbers: Vec::new() }
    }
}

fn accumulate(mut state: State, digits: Vec<SnafuDigit>) -> State {
    // utils::output_into_iter_io(std::io::stdout(), "", &mut digits.iter());
    // println!();
    let value = convert_into_native(&digits);
    let number = Number::new(value, digits);
    state.numbers.push(number);
    state
}

fn reduce(state: State) -> String {
    //Output
    // println!();
    // utils::output_into_iter_io(std::io::stdout(), "\n", &mut state.numbers.iter());
    // println!();
    // println!();

    //Output
    // for num in state.numbers.iter() {
    //     let converted_back = convert_into_snafu(num.value);
    //     utils::output_into_iter_io(std::io::stdout(), "", &mut converted_back.iter());
    //     println!();
    // }
    // println!();
    // println!();

    let sum: i128 = state.numbers.iter().map(|num| num.value).sum();
    let snafu_digits = convert_into_snafu(sum);
    let mut result = String::new();
    utils::output_into_iter(&mut result, "", &mut snafu_digits.iter());
    result
}

const BASE: i128 = 5;

fn convert_into_native(snafu_digits: &Vec<SnafuDigit>) -> i128 {
    let mut value = 0;
    for (place, digit) in snafu_digits.iter().rev().enumerate() {
        let multiplier = BASE.pow(place as u32);
        value += match digit {
            SnafuDigit::Two => multiplier * 2,
            SnafuDigit::One => multiplier,
            SnafuDigit::Zero => 0,
            SnafuDigit::Minus => multiplier * -1,
            SnafuDigit::DoubleMinus => multiplier * -2,
        };
    };
    value
}

fn convert_into_snafu(value: i128) -> Vec<SnafuDigit> {
    // println!("=== Value: {}", value);
    //Special zero case handling
    if value == 0 {
        return vec![SnafuDigit::Zero];
    }
    let mut result = VecDeque::new();
    let mut remaining = value;
    let mut previous_was_adjustment = false;
    while remaining != 0 {
        let mut remainder = remaining.rem_euclid(BASE);
        // println!("remaining: {}", remaining);
        // println!("remainder: {}", remainder);
        if remainder > 2 {
            remainder -= 5;
            // println!("remainder*: {}", remainder);
        }

        if remainder == 0 {
            if !previous_was_adjustment {
                result.push_front(SnafuDigit::Zero);
                // println!("digit: {}", SnafuDigit::Zero);
            }
            remaining /= BASE;
            previous_was_adjustment = false;
        } else {
            let digit = get_snafu_digit(remainder);
            if digit.is_none() {
                panic!("No snafu digit found for remainder {} at value {}", remainder, value);
            }
            // println!("digit: {}", digit.as_ref().unwrap());
            result.push_front(digit.unwrap());
            remaining -= remainder;
            previous_was_adjustment = true;
        }
    };
    Vec::from(result)
}

fn get_snafu_digit(value: i128) -> Option<SnafuDigit> {
    match value {
        -2 => Some(SnafuDigit::DoubleMinus),
        -1 => Some(SnafuDigit::Minus),
        0 => Some(SnafuDigit::Zero),
        1 => Some(SnafuDigit::One),
        2 => Some(SnafuDigit::Two),
        _ => None
    }
}