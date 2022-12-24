use std::collections::HashSet;
use std::mem::swap;

use crate::utils;

const FILE_NAME: &str = "24/input.txt";
//const FILE_NAME: &str = "24/test_input.txt";

pub fn _24a() -> Result<usize, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce1)
}

pub fn _24b() -> Result<usize, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce2)
}

struct Square {
    wall: bool,
    north_wind: bool,
    east_wind: bool,
    south_wind: bool,
    west_wind: bool,
}

impl Square {
    pub fn new_empty() -> Square {
        Square { wall: false, north_wind: false, east_wind: false, south_wind: false, west_wind: false }
    }

    pub fn new_wall() -> Square {
        Square { wall: true, north_wind: false, east_wind: false, south_wind: false, west_wind: false }
    }

    pub fn new_north_wind() -> Square {
        Square { wall: false, north_wind: true, east_wind: false, south_wind: false, west_wind: false }
    }

    pub fn new_east_wind() -> Square {
        Square { wall: false, north_wind: false, east_wind: true, south_wind: false, west_wind: false }
    }

    pub fn new_south_wind() -> Square {
        Square { wall: false, north_wind: false, east_wind: false, south_wind: true, west_wind: false }
    }

    pub fn new_west_wind() -> Square {
        Square { wall: false, north_wind: false, east_wind: false, south_wind: false, west_wind: true }
    }

    fn is_empty(&self) -> bool {
        !self.is_occupied()
    }

    fn is_occupied(&self) -> bool {
        self.wall || self.north_wind || self.east_wind || self.south_wind || self.west_wind
    }

    fn clear_wind(&mut self) {
        self.north_wind = false;
        self.east_wind = false;
        self.south_wind = false;
        self.west_wind = false;
    }
}

fn parse_line(line: String) -> Vec<Square> {
    let mut row: Vec<Square> = Vec::with_capacity(line.len());
    for c in line.chars() {
        let square = match c {
            '#' => Square::new_wall(),
            '^' => Square::new_north_wind(),
            '>' => Square::new_east_wind(),
            'v' => Square::new_south_wind(),
            '<' => Square::new_west_wind(),
            '.' => Square::new_empty(),
            _ => panic!("Unrecognised square '{}'", c),
        };
        row.push(square);
    }
    row
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

struct State {
    num_cols: usize,
    num_rows: usize,
    rows: Vec<Vec<Square>>,
    next_rows: Vec<Vec<Square>>,
}

impl State {
    pub fn new() -> State {
        State {
            num_cols: 0,
            num_rows: 0,
            rows: Vec::new(),
            next_rows: Vec::new(),
        }
    }

    fn get_square(&self, x: usize, y: usize) -> &Square {
        self.rows.get(y).unwrap().get(x).unwrap()
    }
}

fn accumulate(mut state: State, row: Vec<Square>) -> State {
    state.num_cols = row.len();
    let mut next_rows_row = Vec::with_capacity(state.num_cols);
    for square in row.iter() {
        next_rows_row.push(if square.wall {
            Square::new_wall()
        } else {
            Square::new_empty()
        });
    }
    state.next_rows.push(next_rows_row);
    state.rows.push(row);
    state.num_rows += 1;
    state
}

fn reduce1(mut state: State) -> usize {
    let (start, goal) = get_start_and_goal(&state);
    reduce(&mut state, start, goal)
}

fn reduce2(mut state: State) -> usize {
    let (start, goal) = get_start_and_goal(&state);
    let trip_out = reduce(&mut state, start, goal);
    println!("Out: {}", trip_out);
    let return_trip = reduce(&mut state, goal, start);
    println!("Back: {}", return_trip);
    let back_again = reduce(&mut state, start, goal);
    println!("Out Again: {}", back_again);
    trip_out + return_trip + back_again
}

fn get_start_and_goal(state: &State) -> (Coord, Coord) {
    let start_pos = Coord::new(1, 0);
    let mut goal_opt: Option<Coord> = None;
    for x in 0..(state.num_cols) {
        let square = state.get_square(x, state.num_rows - 1);
        if square.is_empty() {
            goal_opt = Some(Coord::new(x, state.num_rows - 1));
            break;
        }
    }
    let goal = goal_opt.expect("Didn't find the goal");
    (start_pos, goal)
}

fn reduce(state: &mut State, start_pos: Coord, goal: Coord) -> usize {
    //Output
    output_state(&state, Some(&start_pos));

    let current_positions: &mut HashSet<Coord> = &mut HashSet::new();
    current_positions.insert(start_pos);
    let new_positions: &mut HashSet<Coord> = &mut HashSet::new();

    let mut steps_required: Option<usize> = None;

    'steps: for step in 0..usize::MAX {
        move_wind(state);

        for current_position in current_positions.drain() {
            let mut choices = Vec::with_capacity(5);
            //choices are north, east, south, west or wait
            if current_position.y > 0 {
                choices.push(Coord::new(current_position.x, current_position.y - 1)); //North
            }
            if current_position.x < state.num_cols - 1 {
                choices.push(Coord::new(current_position.x + 1, current_position.y)); //East
            }
            if current_position.y < state.num_rows - 1 {
                choices.push(Coord::new(current_position.x, current_position.y + 1)); //South
            }
            if current_position.x > 0 {
                choices.push(Coord::new(current_position.x - 1, current_position.y)); //West
            }
            choices.push(Coord::new(current_position.x, current_position.y)); //Wait

            'choices: for choice in choices {
                if choice == start_pos && current_position != start_pos {
                    //no point in going back to the start
                    continue 'choices;
                }
                if choice == goal {
                    steps_required = Some(step);
                    break 'steps;
                }
                let choice_square = state.get_square(choice.x, choice.y);
                if choice_square.is_empty() {
                    //try this square next!!
                    new_positions.insert(choice);
                }
            }
        }

        if new_positions.is_empty() {
            break;
        }

        swap(current_positions, new_positions);

        if step % 10000 == 0 && step > 0 {
            println!("Step {} Completed", step);
        }
    }

    steps_required.map(|steps| steps + 1).expect("Didn't find a route")
}

fn move_wind(state: &mut State) {
    let (num_rows, num_cols) = (state.num_rows, state.num_cols);
    //first (ignoring walls rows and columns) clear the next_rows,
    //these will get updated with the new values and then swapped with the rows
    for y in 1..(num_rows - 1) {
        for x in 1..(num_cols - 1) {
            state.next_rows.get_mut(y).unwrap().get_mut(x).unwrap().clear_wind();
        }
    }
    //ignoring the walls, move the winds
    for y in 1..(num_rows - 1) {
        for x in 1..(num_cols - 1) {
            let square = state.rows.get(y).unwrap().get(x).unwrap();
            move_north_wind(&mut state.next_rows, square, num_rows, x, y);
            move_east_wind(&mut state.next_rows, square, num_cols, x, y);
            move_south_wind(&mut state.next_rows, square, num_rows, x, y);
            move_west_wind(&mut state.next_rows, square, num_cols, x, y);
        }
    }
    //swap over ready for further processing
    swap(&mut state.next_rows, &mut state.rows);
}

fn move_north_wind(next_rows: &mut Vec<Vec<Square>>, square: &Square, num_rows: usize, x: usize, y: usize) {
    if !square.north_wind {
        return;
    }
    let next_square = if y == 1 {
        //wrap around
        next_rows.get_mut(num_rows - 2).unwrap().get_mut(x).unwrap()
    } else {
        next_rows.get_mut(y - 1).unwrap().get_mut(x).unwrap()
    };
    next_square.north_wind = true;
}

fn move_east_wind(next_rows: &mut Vec<Vec<Square>>, square: &Square, num_cols: usize, x: usize, y: usize) {
    if !square.east_wind {
        return;
    }
    let next_square = if x >= num_cols - 2 {
        //wrap around
        next_rows.get_mut(y).unwrap().get_mut(1).unwrap()
    } else {
        next_rows.get_mut(y).unwrap().get_mut(x + 1).unwrap()
    };
    next_square.east_wind = true;
}

fn move_south_wind(next_rows: &mut Vec<Vec<Square>>, square: &Square, num_rows: usize, x: usize, y: usize) {
    if !square.south_wind {
        return;
    }
    let next_square = if y >= num_rows - 2 {
        //wrap around
        next_rows.get_mut(1).unwrap().get_mut(x).unwrap()
    } else {
        next_rows.get_mut(y + 1).unwrap().get_mut(x).unwrap()
    };
    next_square.south_wind = true;
}

fn move_west_wind(next_rows: &mut Vec<Vec<Square>>, square: &Square, num_cols: usize, x: usize, y: usize) {
    if !square.west_wind {
        return;
    }
    let next_square = if x == 1 {
        //wrap around
        next_rows.get_mut(y).unwrap().get_mut(num_cols - 2).unwrap()
    } else {
        next_rows.get_mut(y).unwrap().get_mut(x - 1).unwrap()
    };
    next_square.west_wind = true;
}

fn output_state(state: &State, elf_position: Option<&Coord>) {
    println!();
    println!("{} x {}", state.num_cols, state.num_rows);
    for y in 0..state.num_rows {
        'x_loop: for x in 0..state.num_cols {
            let position = Coord::new(x, y);
            if elf_position.is_some_and(|ep| *ep == position) {
                print!("E");
                continue 'x_loop;
            }
            output_square(state.get_square(x, y));
        }
        println!();
    }
    println!();
}

fn output_square(square: &Square) {
    if square.wall {
        print!("#");
        return;
    }
    let mut winds = 0;
    let mut last_wind: Option<&str> = None;
    if square.north_wind {
        winds += 1;
        last_wind = Some("^");
    }
    if square.east_wind {
        winds += 1;
        last_wind = Some(">");
    }
    if square.south_wind {
        winds += 1;
        last_wind = Some("v");
    }
    if square.west_wind {
        winds += 1;
        last_wind = Some("<");
    }
    if winds == 0 {
        print!(".");
        return
    }
    if winds == 1 {
        print!("{}", last_wind.expect("Missing wind"));
        return;
    }
    print!("{}", winds);
}
