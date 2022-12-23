use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::stdout;
use std::str::Chars;

use either::Either::{self, Left, Right};

use crate::utils;

const FILE_NAME: &str = "22/input.txt";
// const FILE_NAME: &str = "22/test_input.txt";

type Scale = i64;

pub fn _22a() -> Result<Scale, std::io::Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new_empty(),
        accumulate,
        reduce1,
    )
}

pub fn _22b() -> Result<Scale, std::io::Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new_empty(),
        accumulate,
        reduce2,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileState {
    Open,
    Wall,
    OutOfBounds,
}

impl Display for TileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileState::Open => ".",
                TileState::Wall => "#",
                TileState::OutOfBounds => " ",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: Scale,
    y: Scale,
}

impl Coord {
    pub fn new(x: Scale, y: Scale) -> Coord {
        Coord { x, y }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    TurnLeft,
    TurnRight,
    Forward { steps: usize },
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::TurnLeft => write!(f, "L"),
            Move::TurnRight => write!(f, "R"),
            Move::Forward { steps } => write!(f, "{}", steps),
        }
    }
}

///Returns a map of x position (zero based) to state
fn parse_line(line: String) -> Option<Either<HashMap<Scale, TileState>, Vec<Move>>> {
    let mut chars = line.chars();
    let first_char = chars.next();
    match first_char {
        Some(' ') | Some('.') | Some('#') => Some(Left(parse_board_line(first_char, chars))),
        Some(_) => Some(Right(parse_moves(first_char, chars))),
        _ => None,
    }
}

fn parse_board_line(first_char: Option<char>, mut chars: Chars) -> HashMap<Scale, TileState> {
    let mut c = first_char;
    let mut index = 0;
    let mut tile_states = HashMap::new();
    while c.is_some() {
        match c {
            Some(' ') => None,
            Some('.') => tile_states.insert(index, TileState::Open),
            Some('#') => tile_states.insert(index, TileState::Wall),
            _ => panic!("Unrecognised tile char: {}", c.unwrap()),
        };
        c = chars.next();
        index += 1;
    }
    tile_states
}

fn parse_moves(first_char: Option<char>, mut chars: Chars) -> Vec<Move> {
    fn maybe_push_forward(num_string: &mut String, moves: &mut Vec<Move>) {
        if !num_string.is_empty() {
            moves.push(Move::Forward {
                steps: num_string.parse().unwrap(),
            });
            num_string.clear();
        }
    }

    let mut moves = Vec::new();
    let mut num_string = String::new();

    let mut c = first_char;
    while c.is_some() {
        match c {
            Some('R') => {
                maybe_push_forward(&mut num_string, &mut moves);
                moves.push(Move::TurnRight)
            }
            Some('L') => {
                maybe_push_forward(&mut num_string, &mut moves);
                moves.push(Move::TurnLeft)
            }
            Some(num) => num_string.push(num),
            None => {}
        };
        c = chars.next();
    }
    maybe_push_forward(&mut num_string, &mut moves);
    moves
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "^",
                Direction::East => ">",
                Direction::South => "v",
                Direction::West => "<",
            }
        )
    }
}

struct Bounds {
    min_x: Scale,
    max_x: Scale,
    min_y: Scale,
    max_y: Scale,
}

impl Display for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.min_x, self.min_y, self.max_x, self.max_y
        )
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct TileWrap {
    coord: Coord,
    direction: Direction,
}

impl TileWrap {
    pub fn new(x: i64, y: i64, direction: Direction) -> TileWrap {
        TileWrap { coord: Coord::new(x, y), direction }
    }
    pub fn new_coord(coord: Coord, direction: Direction) -> TileWrap {
        TileWrap { coord, direction }
    }
}

struct State {
    rows: Scale,
    bounds: Bounds,
    tiles: HashMap<Coord, TileState>,
    moves: Vec<Move>,
    visited_tiles: HashMap<Coord, Direction>,
    wraps: HashMap<TileWrap, TileWrap>,
}

impl State {
    pub fn new_empty() -> State {
        State {
            rows: 0,
            bounds: Bounds {
                min_x: 0,
                max_x: 0,
                min_y: 0,
                max_y: 0,
            },
            tiles: HashMap::new(),
            moves: Vec::new(),
            visited_tiles: HashMap::new(),
            wraps: HashMap::new(),
        }
    }
}

fn accumulate(
    mut state: State,
    tile_states_or_moves: Option<Either<HashMap<Scale, TileState>, Vec<Move>>>,
) -> State {
    match tile_states_or_moves {
        Some(Left(x_pos_to_state)) => {
            let y = state.rows;
            let mut minx = state.bounds.min_x;
            let mut maxx = state.bounds.max_x;
            for (x, tile_state) in x_pos_to_state {
                minx = cmp::min(minx, x);
                maxx = cmp::max(maxx, x);
                let coord = Coord::new(x, y);
                state.tiles.insert(coord, tile_state);
            }
            State {
                rows: state.rows + 1,
                bounds: Bounds {
                    min_x: cmp::min(state.bounds.min_x, minx),
                    max_x: cmp::max(state.bounds.max_x, maxx),
                    min_y: state.bounds.min_y,
                    max_y: cmp::max(state.bounds.max_y, y),
                },
                ..state
            }
        }
        Some(Right(moves)) => State { moves, ..state },
        None => state,
    }
}

fn reduce1(mut state: State) -> Scale {
    reduce(&mut state, wrapping_function1)
}

fn reduce2(mut state: State) -> Scale {
    //set up the wraps
    set_up_wraps(&mut state, FILE_NAME.contains("test"));
    reduce(&mut state, wrapping_function2)
}

fn reduce(
    state: &mut State,
    wrapping_function: fn(&State, Coord, Direction) -> (Coord, Direction)
) -> Scale {
    output_state(&state);

    let mut coord = find_leftmost_open(&state, 0).unwrap();
    let mut direction = Direction::East;
    let my_moves = state.moves.clone();

    for mv in my_moves.iter() {
        match mv {
            Move::TurnLeft => direction = calculate_new_direction(&direction, Move::TurnLeft),
            Move::TurnRight => direction = calculate_new_direction(&direction, Move::TurnRight),
            Move::Forward { steps } => (coord, direction) = move_forward(state, coord, direction, wrapping_function, *steps),
        }
    }

    println!("Final coord: {}", coord);
    println!("Final direction: {}", direction);

    1000 * (coord.y + 1) + 4 * (coord.x + 1) + direction_value(&direction)
}

/// Calculate the new direction given the current direction and a movement
fn calculate_new_direction(current_direction: &Direction, mv: Move) -> Direction {
    match (current_direction, mv) {
        (Direction::North, Move::TurnLeft) => Direction::West,
        (Direction::East, Move::TurnLeft) => Direction::North,
        (Direction::South, Move::TurnLeft) => Direction::East,
        (Direction::West, Move::TurnLeft) => Direction::South,
        (Direction::North, Move::TurnRight) => Direction::East,
        (Direction::East, Move::TurnRight) => Direction::South,
        (Direction::South, Move::TurnRight) => Direction::West,
        (Direction::West, Move::TurnRight) => Direction::North,
        _ => panic!(
            "Unhandled new direction: {} + {} = ?",
            current_direction, mv
        ),
    }
}

/// Move forwards obeying the rules!
fn move_forward(
    state: &mut State,
    coord: Coord,
    direction: Direction,
    wrapping_function: fn(&State, Coord, Direction) -> (Coord, Direction),
    steps: usize,
) -> (Coord, Direction) {
    //Output
    // println!("Move Forward: {} {} {} steps", coord, direction, steps);

    let mut current_coord = coord.clone();
    let mut current_direction = direction.clone();
    'outer: for _ in 0..steps {
        let (x_inc, y_inc) = get_x_y_increments(&current_direction);
        let mut candidate_coord = Coord::new(current_coord.x + x_inc, current_coord.y + y_inc);
        let mut candidate_direction = current_direction.clone();

        loop {
            //Output
            // println!("candidate: {} {}", candidate_coord, candidate_direction);
            (candidate_coord, candidate_direction) = wrapping_function(state, candidate_coord, candidate_direction);
            // println!("candidate_wrapped: {} {}", candidate_coord, candidate_direction);

            let tile_state = tile_state(state, &candidate_coord);
            // println!("tile_state: {}", tile_state);
            match tile_state {
                TileState::Open => {
                    current_coord = candidate_coord;
                    current_direction = candidate_direction;
                    continue 'outer;
                }
                TileState::Wall => {
                    break 'outer;
                }
                TileState::OutOfBounds => {
                    let (x_inc, y_inc) = get_x_y_increments(&current_direction);
                    candidate_coord.x += x_inc;
                    candidate_coord.y += y_inc;
                    if candidate_coord.x < state.bounds.min_x - 1||
                        candidate_coord.x > state.bounds.max_x + 1 ||
                        candidate_coord.y < state.bounds.min_y - 1 ||
                        candidate_coord.y > state.bounds.max_y + 1
                    {
                        panic!("Candidate is too out of bounds: {} {}", candidate_coord, candidate_direction);
                    }
                }
            }
        }
    }
    (current_coord, current_direction)
}

/// Given a direction of travel, give back the change in co-ordinates for a step in that direction
fn get_x_y_increments(direction: &Direction) -> (Scale, Scale) {
    match direction {
        Direction::North => (0, -1),
        Direction::East => (1, 0),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
    }
}

fn output_state(state: &State) {
    println!("{}", state.bounds);
    for y in state.bounds.min_y..(state.bounds.max_y + 1) {
        for x in state.bounds.min_x..(state.bounds.max_x + 1) {
            let coord = Coord::new(x, y);
            //display last visited first
            match state.visited_tiles.get(&coord) {
                Some(direction) => print!("{}", direction),
                None => match tile_state(state, &Coord::new(x, y)) {
                    state => print!("{}", state),
                },
            }
        }
        println!();
    }
    println!();
    utils::output_into_iter_io(stdout(), "", &mut state.moves.iter());
    println!();
}

fn find_leftmost_open(state: &State, y: Scale) -> Option<Coord> {
    for x in state.bounds.min_x..(state.bounds.max_x + 1) {
        let coord = Coord::new(x, y);
        if tile_state(state, &coord) == TileState::Open {
            return Some(Coord::new(x, y));
        }
    }
    None
}

fn tile_state(state: &State, coord: &Coord) -> TileState {
    *state.tiles.get(coord).unwrap_or(&TileState::OutOfBounds)
}

fn direction_value(direction: &Direction) -> Scale {
    match direction {
        Direction::North => 3,
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
    }
}

fn wrapping_function1(state: &State, mut coord: Coord, direction: Direction) -> (Coord, Direction) {
    //constrain to the bounds
    if coord.x < state.bounds.min_x {
        coord.x = state.bounds.max_x;
    }
    if coord.x > state.bounds.max_x {
        coord.x = state.bounds.min_x;
    }
    if coord.y < state.bounds.min_y {
        coord.y = state.bounds.max_y;
    }
    if coord.y > state.bounds.max_y {
        coord.y = state.bounds.min_y;
    }
    (coord, direction)
}

fn wrapping_function2(state: &State, coord: Coord, direction: Direction) -> (Coord, Direction) {
    //wrap if it's set up to do it
    match state.wraps.get(&TileWrap::new_coord(coord, direction)) {
        Some(wrap) => (wrap.coord, wrap.direction),
        None => (coord, direction),
    }
}

fn set_up_wraps(state: &mut State, test_input: bool) {
    //test_input
    if test_input {
        set_up_test_wraps(state);
    } else {
        set_up_main_wraps(state);
    }
}

/// grid is in the following format
///  12
///  3
/// 45
/// 6
///
/// FIXME:  We should only need to define the mapping once and derive the reverse, i.e.
/// if 1N -> 3E then we know 3W -> 1S
fn set_up_main_wraps(state: &mut State) {
    let side_length = 50;
    // 3 west to 4 south
    setup_tile_wraps(state, side_length,
        1, 1, Direction::West,
        0, 2, Direction::South, false
    );
    // 4 north to 3 east
    setup_tile_wraps(state, side_length,
        0, 2, Direction::North,
        1, 1, Direction::East, false
    );
    // 3 east to 2 north
    setup_tile_wraps(state, side_length,
        1, 1, Direction::East,
        2, 0, Direction::North, false
    );
    // 2 south to 3 west
    setup_tile_wraps(state, side_length,
        2, 0, Direction::South,
        1, 1, Direction::West, false
    );
    // 5 south to 6 west
    setup_tile_wraps(state, side_length,
        1, 2, Direction::South,
        0, 3, Direction::West, false
    );
    // 6 east to 5 north
    setup_tile_wraps(state, side_length,
        0, 3, Direction::East,
        1, 2, Direction::North, false
    );
    // 2 east to 5 west
    setup_tile_wraps(state, side_length,
        2, 0, Direction::East,
        1, 2, Direction::West, true
    );
    // 5 east to 2 west
    setup_tile_wraps(state, side_length,
        1, 2, Direction::East,
        2, 0, Direction::West, true
    );
    // 1 west to 4 east
    setup_tile_wraps(state, side_length,
        1, 0, Direction::West,
        0, 2, Direction::East, true
    );
    // 4 west to 1 east
    setup_tile_wraps(state, side_length,
        0, 2, Direction::West,
        1, 0, Direction::East, true
    );
    // 1 north to 6 east
    setup_tile_wraps(state, side_length,
        1, 0, Direction::North,
        0, 3, Direction::East, false
    );
    // 6 west to 1 south
    setup_tile_wraps(state, side_length,
        0, 3, Direction::West,
        1, 0, Direction::South, false
    );
    // 2 north to 6 north
    setup_tile_wraps(state, side_length,
        2, 0, Direction::North,
        0, 3, Direction::North, false
    );
    // 6 south to 2 south
    setup_tile_wraps(state, side_length,
        0, 3, Direction::South,
        2, 0, Direction::South, false
    );
}

/// FIXME:  Convert all of these to use setup_tile_wraps.
fn set_up_test_wraps(state: &mut State) {
    let side_length = 4;
    //North
    //1 north to 2 south
    setup_tile_wraps(state, side_length,
        3, 0, Direction::North,
        0, 1, Direction::South,
        true
    );
    //2 north to 1 south
    setup_tile_wraps(state, side_length,
        0, 1, Direction::North,
        2, 0, Direction::South,
        true
    );
    //3 north to 1 east
    setup_tile_wraps(state, side_length,
        1, 1, Direction::North,
        2, 0, Direction::East,
        false
    );
    //6 north to 4 west
    setup_tile_wraps(state, side_length,
        3, 2, Direction::North,
        2, 1, Direction::West,
        false
    );
    //South
    //2 south to 5 north
    for x in 0..side_length {
        state.wraps.insert(
            TileWrap::new(x, side_length * 2, Direction::South),
            TileWrap::new(side_length * 3 - 1 - x, side_length * 3 - 1, Direction::North)
        );
    }
    //3 south to 5 east
    for x in 0..side_length {
        state.wraps.insert(
            TileWrap::new(side_length + x, side_length * 2, Direction::South),
            TileWrap::new(side_length * 2, side_length * 3 - 1 - x, Direction::East)
        );
    }
    //5 south to 2 north
    for x in 0..side_length {
        state.wraps.insert(
            TileWrap::new(side_length * 2 + x, side_length * 3, Direction::South),
            TileWrap::new(side_length - 1 - x, side_length * 2 - 1, Direction::North)
        );
    }
    //6 south to 2 East
    for x in 0..side_length {
        state.wraps.insert(
            TileWrap::new(side_length * 3 + x, side_length * 3, Direction::South),
            TileWrap::new(0, side_length * 2 - 1 - x, Direction::East)
        );
    }
    //east
    //1 east to 6 west
    for y in 0..side_length {
        state.wraps.insert(
            TileWrap::new(side_length * 3, y, Direction::East),
            TileWrap::new(side_length * 4 - 1, side_length * 3 - 1 - y, Direction::West)
        );
    }
    //4 east to 6 south
    for y in 0..side_length {
        state.wraps.insert(
            TileWrap::new(side_length * 3, side_length + y, Direction::East),
            TileWrap::new(side_length * 4 - 1 - y, side_length * 2, Direction::South)
        );
    }
    //6 east to 1 west
    for y in 0..side_length {
        state.wraps.insert(
            TileWrap::new(side_length * 4, side_length * 2 + y, Direction::East),
            TileWrap::new(side_length * 3 - 1, side_length - 1 - y, Direction::South)
        );
    }
    //west
    //1 west to 3 south
    for y in 0..side_length {
        state.wraps.insert(
            TileWrap::new(side_length * 2 - 1, y, Direction::West),
            TileWrap::new(side_length * 2 - 1 - y, side_length, Direction::South)
        );
    }
    //2 west to 6 north
    for y in 0..side_length {
        state.wraps.insert(
            TileWrap::new(- 1, side_length + y, Direction::West),
            TileWrap::new(side_length * 4 - 1 - y, side_length * 3 - 1, Direction::North)
        );
    }
    //5 west to 3 north
    for y in 0..side_length {
        state.wraps.insert(
            TileWrap::new(side_length * 2 - 1, side_length * 2 + y, Direction::West),
            TileWrap::new(side_length * 2 - 1 - y, side_length * 2 - 1, Direction::North)
        );
    }
}

//FIXME:  Do the converse directions automatically to avoid setting up the opposites
fn setup_tile_wraps(state: &mut State, side_length: Scale,
    from_grid_x: Scale, from_grid_y: Scale, from_direction: Direction,
    to_grid_x: Scale, to_grid_y: Scale, to_direction: Direction,
    reverse: bool)
{
    let (from_x_min, from_x_max_plus_1, from_y_min, from_y_max_plus_1) = match from_direction {
        Direction::North => (
            side_length * from_grid_x,
            side_length * (from_grid_x + 1),
            side_length * from_grid_y - 1,
            side_length * from_grid_y
        ),
        Direction::East => (
            side_length * (from_grid_x + 1),
            side_length * (from_grid_x + 1) + 1,
            side_length * from_grid_y,
            side_length * (from_grid_y + 1)
        ),
        Direction::South => (
            side_length * from_grid_x,
            side_length * (from_grid_x + 1),
            side_length * (from_grid_y + 1),
            side_length * (from_grid_y + 1) + 1
        ),
        Direction::West => (
            side_length * from_grid_x - 1,
            side_length * from_grid_x,
            side_length * from_grid_y,
            side_length * (from_grid_y + 1)
        ),
    };
    let (to_start_x, inc_x, to_start_y, inc_y) = match (to_direction, reverse) {
        (Direction::North, false) => (
            side_length * to_grid_x, 1,
            side_length * (to_grid_y + 1) - 1, 0
        ),
        (Direction::North, true) => (
            side_length * (to_grid_x + 1) - 1, -1,
            side_length * (to_grid_y + 1) - 1, 0
        ),
        (Direction::East, false) => (
            side_length * to_grid_x, 0,
            side_length * to_grid_y, 1
        ),
        (Direction::East, true) => (
            side_length * to_grid_x, 0,
            side_length * (to_grid_y + 1) - 1, -1
        ),
        (Direction::South, false) => (
            side_length * to_grid_x, 1,
            side_length * to_grid_y, 0
        ),
        (Direction::South, true) => (
            side_length * (to_grid_x + 1) - 1, -1,
            side_length * to_grid_y, 0
        ),
        (Direction::West, false) => (
            side_length * (to_grid_x + 1) - 1, 0,
            side_length * to_grid_y, 1
        ),
        (Direction::West, true) => (
            side_length * (to_grid_x + 1) - 1, 0,
            side_length * (to_grid_y + 1) - 1, -1
        ),
    };
    let mut dest_x = to_start_x;
    let mut dest_y = to_start_y;
    for source_x in from_x_min..from_x_max_plus_1 {
        for source_y in from_y_min..from_y_max_plus_1 {
            state.wraps.insert(
                TileWrap::new(source_x, source_y, from_direction),
                TileWrap::new(dest_x, dest_y, to_direction)
            );
            dest_x += inc_x;
            dest_y += inc_y;
        }
    }
}