use std::io::Error;
use std::collections::HashSet;

use substring::Substring;

use crate::utils;

const FILE_NAME: &str = "10/input.txt";

pub fn _10a_and_10b() -> Result<i64, Error> {
    utils::process_file(
        FILE_NAME,
        parse_func,
        Signal::new(HashSet::from([20, 60, 100, 140, 180, 220])),
        accumulator,
        reducer
    )
}

enum Instruction {
    AddX { value: i64 },
    Noop,
}

fn parse_func(line: String) -> Option<Instruction> {
    let initial = line.substring(0, 4);
    match initial {
        "addx" => Some(Instruction::AddX {
            value: line.substring(5, line.len()).parse().unwrap()
        }),
        "noop" => Some(Instruction::Noop),
        _ => None,
    }
}

struct Signal {
    x: i64,
    cycle: i64,
    interesting_cycles: HashSet<i64>,
    sampled_values: Vec<i64>
}

impl Signal {
    pub fn new(interesting_cycles: HashSet<i64>) -> Signal {
        let len = interesting_cycles.len();
        Signal {
            x: 1,
            cycle: 1,
            interesting_cycles: interesting_cycles,
            sampled_values: Vec::with_capacity(len),
        }
    }
}

fn accumulator(mut signal: Signal, instruction: Option<Instruction>) -> Signal {
    match instruction {
        Some(Instruction::Noop) => {
            next_cycle(&mut signal);
            signal
        },
        Some(Instruction::AddX { value }) => {
            next_cycle(&mut signal);
            next_cycle(&mut signal);
            signal.x += value;
            signal
        },
        _ => signal,
    }
}

fn next_cycle(signal: &mut Signal) {
    maybe_sample_value(signal);
    output_pixel(&signal);
    signal.cycle += 1;
}

fn maybe_sample_value(signal: &mut Signal) {
    if signal.interesting_cycles.contains(&signal.cycle) {
        let sampled_value = signal.cycle * signal.x;
        signal.sampled_values.push(sampled_value);
    }
}

fn output_pixel(signal: &Signal) {
    let horz_pos = (signal.cycle - 1) % 40;
    //Move to next line?
    if horz_pos == 0 {
        print!("\n")
    }
    //draw pixel
    if is_in_sprite(horz_pos, signal) {
        print!("#");
    } else {
        print!(".");
    }
}

fn is_in_sprite(horz_pixel: i64, signal: &Signal) -> bool {
    horz_pixel >= signal.x - 1 && horz_pixel <= signal.x + 1
}

fn reducer(mut signal: Signal) -> i64 {
    maybe_sample_value(&mut signal); //just incase we have the very last as an interesting value
    print!("\n");
    print!("\n");
    signal.sampled_values.iter().fold(0, |acc, value| acc + value)
}
