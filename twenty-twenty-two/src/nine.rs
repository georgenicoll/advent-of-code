use std::collections::HashSet;
use std::io::Error;
use substring::Substring;

use crate::utils;


const FILE_NAME: &str = "9/input.txt";

pub fn _9a() -> Result<u64, Error>{
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new(2),
        accumulate,
        reduce
    )
}

pub fn _9b() -> Result<u64, Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new(10),
        accumulate,
        reduce
    )
}

enum Move {
    Up{ num: u8 },
    Down{ num: u8 },
    Left{ num: u8 },
    Right{ num: u8 },
}

fn parse_line(line: String) -> Option<Move> {
    match line.chars().next() {
        Some('U') => Some(Move::Up { num: parse_num(&line) }),
        Some('D') => Some(Move::Down { num: parse_num(&line) }),
        Some('L') => Some(Move::Left { num: parse_num(&line) }),
        Some('R') => Some(Move::Right { num: parse_num(&line) }),
        _ => None,
    }
}

fn parse_num(line: &String) -> u8 {
    //drop the first 2 chars, this is the direction and a space
    let number_string = line.substring(2, line.len());
    number_string.parse().unwrap()
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Position {
        Position { x, y }
    }
}

#[derive(Debug)]
struct State {
    num_positions: usize,
    positions: Vec<Position>,
    visited_positions: HashSet<Position>,
}

impl State {
    pub fn new(num_positions: usize) -> State {
        let mut positions = Vec::with_capacity(num_positions);
        for _ in 0..num_positions {
            positions.push(Position::new(0, 0));
        }
        State {
            num_positions: num_positions,
            positions: positions,
            visited_positions: HashSet::from([Position::new(0, 0)]),
        }
    }
}

fn accumulate(state: State, head_move: Option<Move>) -> State {
    match head_move {
        Some(Move::Up{ num }) => perform_moves(state, num, 0, 1),
        Some(Move::Down{ num }) => perform_moves(state, num, 0, -1),
        Some(Move::Left{ num }) => perform_moves(state, num, -1, 0),
        Some(Move::Right{ num }) => perform_moves(state, num, 1, 0),
        None => state,
    }
}

fn perform_moves(mut state: State, num: u8, horz_move: i16, vert_move: i16) -> State {
    for _ in 0..num {
        let mut moved = false;
        {
            let first = state.positions.first_mut().unwrap();
            first.x += horz_move;
            first.y += vert_move;
        }

        for position_index in 0..state.num_positions - 1 {
            let (pos1_x, pos1_y) = {
                let pos1 = state.positions.get(position_index).unwrap();
                (pos1.x, pos1.y)
            };
            let pos2 = state.positions.get_mut(position_index + 1).unwrap();

            //decide if we need to move the second position
            let horz_diff = (pos1_x - pos2.x).abs();
            let vert_diff = (pos1_y - pos2.y).abs();

            let horz_bump = if pos1_x > pos2.x {
                1
            } else {
                -1
            };
            let vert_bump = if pos1_y > pos2.y {
                1
            } else {
                -1
            };

            moved = match (horz_diff, vert_diff) {
                (0, 0) => false,
                (1, 0) => false,
                (0, 1) => false,
                (1, 1) => false,
                (2, 2) => {
                    pos2.x += horz_bump;
                    pos2.y += vert_bump;
                    true
                }
                (_, 0) => {
                    pos2.x += horz_bump;
                    true
                }
                (_, 1) => {
                    pos2.x += horz_bump;
                    pos2.y += vert_bump;
                    true
                },
                (0, _) => {
                    pos2.y += vert_bump;
                    true
                }
                (1, _) => {
                    pos2.x += horz_bump;
                    pos2.y += vert_bump;
                    true
                }
                _ => panic!("Unexpected difference between positions: ({}, {})", horz_diff, vert_diff)
            };
        }
        if moved {
            let last = state.positions.last().unwrap();
            state.visited_positions.insert(Position { x: last.x, y: last.y });
        }
    }
    state
}

fn reduce(state: State) -> u64 {
    state.visited_positions.len() as u64
}

