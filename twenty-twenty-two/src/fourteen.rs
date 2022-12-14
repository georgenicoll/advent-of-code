use std::fmt::Display;
use std::collections::HashMap;
use std::cmp::{min, max};
use std::fs::File;
use std::io::{BufWriter, Write};

use lazy_static::lazy_static;

use crate::utils;

const FILE_NAME: &str = "14/input.txt";

pub fn _14a() -> Result<u32, std::io::Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new(),
        accumulate,
        reduce1
    )
}

pub fn _14b() -> Result<u32, std::io::Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new(),
        accumulate,
        reduce2
    )
}

#[derive(Debug, Clone, Copy)]
struct Link {
    x: Scale,
    y: Scale,
}

impl Link {
    pub fn new(x: Scale, y: Scale) -> Link {
        Link { x, y }
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn parse_line(line: String) -> Vec<Link> {
    let mut links = Vec::with_capacity(10);
    let mut x_opt: Option<Scale> = None;
    let mut num = String::new();
    for c in line.chars() {
        match c {
            ',' => {
                x_opt = Some(num.parse().unwrap());
                num.clear()
            },
            ' ' => {
                if let Some(x) = x_opt {
                    let y: Scale = num.parse().unwrap();
                    num.clear();
                    links.push(Link::new(x, y));
                    x_opt = None
                }
            },
            '-' | '>' => {}, //ignore these
            _ => num.push(c),
        }
    }
    //final link needs to be created
    if let Some(x) = x_opt {
        let y = num.parse().unwrap();
        links.push(Link::new(x, y));
    }
    links
}

type Scale = i32;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: Scale,
    y: Scale,
}

impl Pos {
    pub fn new(x: Scale, y: Scale) -> Pos {
        Pos{x, y}
    }
}

enum Blockage {
    Rock,
    Sand
}

struct State {
    blockages: HashMap<Pos, Blockage>,
}

impl State {
    pub fn new() -> State {
        State{ blockages: HashMap::new() }
    }
}

// fn output_links(links: &Vec<Link>) {
//     let mut link_iter = links.iter();
//     if let Some(link) = link_iter.next() {
//         print!("{}", link);
//     }
//     for link in link_iter {
//         print!(" -> {}", link);
//     }
//     println!("");
// }

///returns the bounds of the rocks (min_x, min_y, max_x, max_y)
fn calc_bounds(state: &State) -> (Scale, Scale, Scale, Scale) {
    let mut min_x = Scale::MAX;
    let mut min_y = Scale::MAX;
    let mut max_x = Scale::MIN;
    let mut max_y = Scale::MIN;
    for pos in state.blockages.keys() {
        min_x = min(pos.x, min_x);
        min_y = min(pos.y, min_y);
        max_x = max(pos.x, max_x);
        max_y = max(pos.y, max_y);
    }
    (min_x, min_y, max_x, max_y)
}


fn output_state(state: &State, file_name: &str) {
    lazy_static! {
        static ref BLOCK: String = String::from("#");
        static ref SAND: String = String::from("o");
        static ref EMPTY: String = String::from(".");
    }
    File::create(file_name).map(BufWriter::new).map(|mut writer| {
        let (min_x, min_y, max_x, max_y) = calc_bounds(state);
        writeln!(writer, "{}-{}x{}-{}", min_x, max_x, min_y, max_y).unwrap();
        for y in min_y..(max_y + 1) {
            for x in min_x..(max_x + 1) {
                let pos = Pos::new(x, y);
                match state.blockages.get(&pos) {
                    Some(Blockage::Rock) => writer.write(BLOCK.as_bytes()),
                    Some(Blockage::Sand) => writer.write(SAND.as_bytes()),
                    None => writer.write(EMPTY.as_bytes()),
                }.unwrap();
            }
            writeln!(writer).unwrap();
        }
        writer.flush().unwrap();
    }).unwrap();
}

fn accumulate(mut state: State, links: Vec<Link>) -> State {
    // output_links(links);
    let mut link_iter = links.iter();
    let mut prev_link = link_iter.next().unwrap();
    for link in link_iter {
        if prev_link.x == link.x {
            for y in min(prev_link.y, link.y)..(max(prev_link.y, link.y) + 1) {
                state.blockages.insert(Pos::new(link.x, y), Blockage::Rock);
            }
        } else if prev_link.y == link.y {
            for x in min(prev_link.x, link.x)..(max(prev_link.x, link.x) + 1) {
                state.blockages.insert(Pos::new(x, link.y), Blockage::Rock);
            }
        } else {
            panic!("Expecting links to have the same x or y: {} -> {}", prev_link, link);
        }
        prev_link = link;
    }
    //output_state(&state, "14/accumulate-output.txt");
    state
}

const START_X: Scale = 500;
const START_Y: Scale = 0;

fn reduce1(mut state: State) -> u32 {
    // output_state(&state, "14/reduce1-start-output.txt");

    let (_, _, _, escape_y) = calc_bounds(&state);

    fn start_pos() -> Pos {
        Pos::new(START_X, START_Y)
    }

    let mut pos = start_pos();
    let mut grains_at_rest: u32 = 0;
    loop {
        if !move_down(&state, &mut pos) {
            if !move_down_left(&state, &mut pos) {
                if !move_down_right(&state, &mut pos) {
                    //unable to move - new grain at rest and add a blockage
                    grains_at_rest += 1;
                    state.blockages.insert(pos, Blockage::Sand);
                    pos = start_pos();
                }
            }
        }
        //check whether this grain is free - if so, we are done
        if pos.y >= escape_y {
            break;
        }
    }

    output_state(&state, "14/reduce1-output.txt");

    grains_at_rest
}

fn reduce2(mut state: State) -> u32 {
    // output_state(&state);

    let (_, _, _, lowest_rock) = calc_bounds(&state);
    let lowest_y = lowest_rock + 1;

    fn start_pos() -> Pos {
        Pos::new(START_X, START_Y)
    }

    let mut pos = start_pos();
    let mut grains_at_rest: u32 = 0;
    loop {
        //floor check
        if pos.y == lowest_y {
            grains_at_rest += 1;
            state.blockages.insert(pos, Blockage::Sand);
            pos = start_pos();
            continue;
        }

        if !move_down(&state, &mut pos) {
            if !move_down_left(&state, &mut pos) {
                if !move_down_right(&state, &mut pos) {
                    //is source blocked, in which case stop
                    //unable to move - new grain at rest and add a blockage
                    grains_at_rest += 1;
                    state.blockages.insert(pos, Blockage::Sand);
                    if pos.y == START_Y {
                        break;
                    }
                    pos = start_pos();
                }
            }
        }
    }

    let (min_x, _, max_x, _) = calc_bounds(&state);
    //draw the floor
    for x in min_x..(max_x + 1) {
        state.blockages.insert(Pos::new(x, lowest_rock + 2), Blockage::Rock);
    }
    output_state(&state, "14/reduce2-output.txt");

    grains_at_rest
}

fn move_down(state: &State, pos: &mut Pos) -> bool {
    //see if we can move down
    pos.y += 1;
    match state.blockages.get(pos) {
        Some(_) => {
            //blocked - put it back
            pos.y -= 1;
            false
        },
        None => true //ok, it's moved
    }
}

fn move_down_left(state: &State, pos: &mut Pos) -> bool {
    //see if we can move down and left
    pos.x -= 1;
    pos.y += 1;
    match state.blockages.get(pos) {
        Some(_) => {
            //blocked - put it back
            pos.x += 1;
            pos.y -= 1;
            false
        },
        None => true //ok, it's moved
    }
}

fn move_down_right(state: &State, pos: &mut Pos) -> bool {
    //see if we can move down and right
    pos.x += 1;
    pos.y += 1;
    match state.blockages.get(pos) {
        Some(_) => {
            //blocked - put it back
            pos.x -= 1;
            pos.y -= 1;
            false
        },
        None => true //ok, it's moved
    }
}