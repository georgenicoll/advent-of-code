use std::cmp;
use std::fmt::Display;
use std::collections::{HashSet, HashMap};

use crate::utils;

const FILE_NAME: &str = "23/input.txt";
// const FILE_NAME: &str = "23/test_input.txt";

pub fn _23a() -> Result<usize, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce1)
}

pub fn _23b() -> Result<usize, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce2)
}

fn parse_line(line: String) -> Vec<i32> {
    let chars = line.chars();
    chars
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .fold(Vec::new(), |mut acc, (index, _)| {
            acc.push(index as i32);
            acc
        })
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    pub fn surrounding_coords(&self) -> [Coord; 8] {
        [
            Coord::new(self.x - 1, self.y - 1),
            Coord::new(self.x, self.y - 1),
            Coord::new(self.x + 1, self.y -1),
            Coord::new(self.x - 1, self.y),
            Coord::new(self.x + 1, self.y),
            Coord::new(self.x - 1, self.y + 1),
            Coord::new(self.x, self.y + 1),
            Coord::new(self.x + 1, self.y + 1),
        ]
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Bounds {
    pub fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Bounds {
        Bounds { min_x, max_x, min_y, max_y }
    }
}

struct State {
    rows: i32,
    elf_positions: HashSet<Coord>,
}

impl State {
    pub fn new() -> State {
        State {
            rows: 0,
            elf_positions: HashSet::new(),
        }
    }
}

fn accumulate(mut state: State, elf_x_positions: Vec<i32>) -> State {
    elf_x_positions.iter().for_each(|x_pos| {
        state.elf_positions.insert(Coord::new(*x_pos, state.rows));
    });
    state.rows += 1;
    state
}

const ROUNDS: usize = 10;

enum ProposalDetails {
    MultipleProposals,
    OldPosition{ old_coord: Coord }
}

fn reduce1(mut state: State) -> usize {
    reduce(&mut state, ROUNDS)
}

fn reduce2(mut state: State) -> usize {
    reduce(&mut state, usize::MAX)
}

fn reduce(state: &mut State, rounds: usize) -> usize {
    output_state(&state);

    //Map from proposed position to ProposalDetails
    let mut proposed_positions: HashMap<Coord, ProposalDetails> = HashMap::new();
    'outer: for round in 0..rounds {

        'inner: for elf in state.elf_positions.iter() {
            if nothing_around(&state, elf) {
                continue 'inner;
            }
            propose_position(&state, elf, round, &mut proposed_positions);
        }

        //break if nothing to do
        if proposed_positions.is_empty() {
            println!("Stopped at round {}: Nothing to do", round);
            break 'outer;
        }

        //make the prosed moves
        for (proposed_coord, old_position_or_multiple) in proposed_positions.iter() {
            match old_position_or_multiple {
                ProposalDetails::OldPosition{old_coord} => {
                    state.elf_positions.remove(&old_coord);
                    state.elf_positions.insert(*proposed_coord);
                },
                ProposalDetails::MultipleProposals => {},
            }
        }

        //Output
        // println!("Round {}:", round + 1);
        // output_state(&state);
        // println!();

        proposed_positions.clear();
    }

    let bounds = calculate_bounds(state);
    let grid_squares = ((bounds.max_x - bounds.min_x + 1) * (bounds.max_y - bounds.min_y + 1)) as usize;
    grid_squares - state.elf_positions.len()
}

fn nothing_around(state: &State, elf: &Coord) -> bool {
    coords_are_empty(state, &elf.surrounding_coords())
}

fn coords_are_empty(state: &State, coords: &[Coord]) -> bool {
    coords.iter().fold(true, |acc, coord| acc && !state.elf_positions.contains(coord))
}

type ProposalFunction = fn (&State, &Coord) -> Option<Coord>;

const NUM_PROPOSAL_FUNCTIONS: usize = 4;
const PROPOSAL_FUNCTIONS: [ProposalFunction; NUM_PROPOSAL_FUNCTIONS] = [
    north_proposal,
    south_proposal,
    west_proposal,
    east_proposal,
];

fn propose_position(state: &State, elf: &Coord, round: usize, proposed_positions: &mut HashMap<Coord, ProposalDetails>) {
    //look at each of the directions in the specific order for the round stopping at the first proposal
    let mut proposal = None;
    for proposal_num in 0..NUM_PROPOSAL_FUNCTIONS {
        let proposal_function_num = (proposal_num + round) % NUM_PROPOSAL_FUNCTIONS;
        let proposal_function = PROPOSAL_FUNCTIONS[proposal_function_num];
        proposal = proposal_function(state, elf);
        if proposal.is_some() {
            break;
        };
    }
    //did we manage to propose anything?
    if let Some(proposed_coord) = proposal {
        //put the proposal in, or if there are now multiple - replace with the multiple marker
        let existing_proposal = proposed_positions.get(&proposed_coord);
        match existing_proposal {
            None => {
                proposed_positions.insert(proposed_coord, ProposalDetails::OldPosition { old_coord: elf.clone() });
            },
            Some(ProposalDetails::OldPosition { old_coord: _ }) => {
                proposed_positions.insert(proposed_coord, ProposalDetails::MultipleProposals);
            },
            Some(ProposalDetails::MultipleProposals) => {},
        };
    };
}

const NORTH_ADJUSTMENTS: [Coord; 3] = [
    Coord{ x: -1, y: -1 },
    Coord{ x: 0, y: -1 },
    Coord{ x: 1, y: -1 },
];

fn north_proposal(state: &State, elf: &Coord) -> Option<Coord> {
    if coords_are_empty(state, &get_adjused_coords(elf, &NORTH_ADJUSTMENTS)) {
        Some(Coord::new(elf.x, elf.y - 1))
    } else {
        None
    }
}

const SOUTH_ADJUSTMENTS: [Coord; 3] = [
    Coord{ x: -1, y: 1 },
    Coord{ x: 0, y: 1 },
    Coord{ x: 1, y: 1 },
];

fn south_proposal(state: &State, elf: &Coord) -> Option<Coord> {
    if coords_are_empty(state, &get_adjused_coords(elf, &SOUTH_ADJUSTMENTS)) {
        Some(Coord::new(elf.x, elf.y + 1))
    } else {
        None
    }
}

const WEST_ADJUSTMENTS: [Coord; 3] = [
    Coord{ x: -1, y: -1 },
    Coord{ x: -1, y: 0 },
    Coord{ x: -1, y: 1 },
];

fn west_proposal(state: &State, elf: &Coord) -> Option<Coord> {
    if coords_are_empty(state, &get_adjused_coords(elf, &WEST_ADJUSTMENTS)) {
        Some(Coord::new(elf.x - 1, elf.y))
    } else {
        None
    }
}

const EAST_ADJUSTMENTS: [Coord; 3] = [
    Coord{ x: 1, y: -1 },
    Coord{ x: 1, y: 0 },
    Coord{ x: 1, y: 1 },
];

fn east_proposal(state: &State, elf: &Coord) -> Option<Coord> {
    if coords_are_empty(state, &get_adjused_coords(elf, &EAST_ADJUSTMENTS)) {
        Some(Coord::new(elf.x + 1, elf.y))
    } else {
        None
    }
}

fn get_adjused_coords(elf: &Coord, adjustments: &[Coord; 3]) -> [Coord; 3] {
    adjustments.map(|adjustment| {
        Coord::new(elf.x + adjustment.x, elf.y + adjustment.y)
    })
}

fn output_state(state: &State) {
    let bounds = calculate_bounds(state);
    for y in bounds.min_y..(bounds.max_y + 1) {
        for x in bounds.min_x..(bounds.max_x + 1) {
            if state.elf_positions.contains(&Coord::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn calculate_bounds(state: &State) -> Bounds {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for coord in state.elf_positions.iter() {
        min_x = cmp::min(min_x, coord.x);
        max_x = cmp::max(max_x, coord.x);
        min_y = cmp::min(min_y, coord.y);
        max_y = cmp::max(max_y, coord.y);
    }
    Bounds::new(min_x, max_x, min_y, max_y)
}
