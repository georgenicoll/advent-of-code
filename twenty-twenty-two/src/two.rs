use std::io::Error;
use crate::utils;

pub fn _2a() -> Result<i64, Error>{
    run_calc(parse_line_1)
}

pub fn _2b() -> Result<i64, Error>{
    run_calc(parse_line_2)
}

fn run_calc(parse_line_fun: fn (String) -> Option<(i64, i64)>) -> Result<i64, Error>{
    let filename = "2/input.txt";
    utils::process_file(
        filename,
        parse_line_fun,
        (0, 0),
        accumulate,
        reduce
    )
}

fn accumulate(acc: (i64, i64), next: Option<(i64, i64)>) -> (i64, i64) {
    match next {
        Some((a, b)) => (acc.0 + a, acc.1 + b),
        None => acc,
    }
}

fn reduce(acc: (i64, i64)) -> i64 {
    acc.1
}

enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Aim {
    Lose,
    Draw,
    Win
}

fn parse_line_1(line: String) -> Option<(i64, i64)> {
    base_parse_line(line, play1)
}

fn play1(p1: char, p2: char) -> (i64, i64) {
    let play_1 = get_play(p1);
    let play_2 = get_play(p2);
    match (play_1, play_2) {
        (Play::Rock, Play::Rock) => (1 + 3, 1 + 3),
        (Play::Rock, Play::Paper) => (1 + 0, 2 + 6),
        (Play::Rock, Play::Scissors) => (1 + 6, 3 + 0),
        (Play::Paper, Play::Rock) => (2 + 6, 1 + 0),
        (Play::Paper, Play::Paper) => (2 + 3, 2 + 3),
        (Play::Paper, Play::Scissors) => (2 + 0, 3 + 6),
        (Play::Scissors, Play::Rock) => (3 + 0, 1 + 6),
        (Play::Scissors, Play::Paper) => (3 + 6, 2 + 0),
        (Play::Scissors, Play::Scissors) => (3 + 3, 3 + 3),
    }
}

fn parse_line_2(line: String) -> Option<(i64, i64)> {
    base_parse_line(line, play2)
}

fn play2(p1: char, p2: char) -> (i64, i64) {
    let play = get_play(p1);
    let aim = get_aim(p2);
    match (play, aim) {
        (Play::Rock, Aim::Lose) => (1 + 6, 3 + 0),
        (Play::Rock, Aim::Draw) => (1 + 3, 1 + 3),
        (Play::Rock, Aim::Win) => (1 + 0, 2 + 6),
        (Play::Paper, Aim::Lose) => (2 + 6, 1 + 0),
        (Play::Paper, Aim::Draw) => (2 + 3, 2 + 3),
        (Play::Paper, Aim::Win) => (2 + 0, 3 + 6),
        (Play::Scissors, Aim::Lose) => (3 + 6, 2 + 0),
        (Play::Scissors, Aim::Draw) => (3 + 3, 3 + 3),
        (Play::Scissors, Aim::Win) => (3 + 0, 1 + 6),
    }
}

fn get_play(p: char) -> Play {
    match p {
        'A' | 'X' => Play::Rock,
        'B' | 'Y' => Play::Paper,
        'C' | 'Z' => Play::Scissors,
        _ => panic!("Unrecognised play {}", p),
    }
}

fn get_aim(p: char) -> Aim {
    match p {
        'X' => Aim::Lose,
        'Y' => Aim::Draw,
        'Z' => Aim::Win,
        _ => panic!("Unrecognised aim {}", p),
    }
}

fn base_parse_line(line: String, cost_func: fn (char, char) -> (i64, i64))  -> Option<(i64, i64)> {
    let mut chars = line.chars();
    let p1_opt = chars.next();
    let space_opt = chars.next();
    let p2_opt = chars.next();
    match (p1_opt, space_opt, p2_opt) {
        (Some(p1), Some(' '), Some(p2)) => Some(cost_func(p1, p2)),
        _ => None,
    }
}
