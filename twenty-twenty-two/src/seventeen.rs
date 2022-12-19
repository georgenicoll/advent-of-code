use std::{str::Chars, collections::{HashSet, BTreeMap}};

use crate::utils;

const FILE_NAME: &str = "17/input.txt";
//const FILE_NAME: &str = "17/test_input.txt";

pub fn _17a() -> Result<i64, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce1)
}

pub fn _17b() -> Result<i64, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce2)
}

fn parse_line(line: String) -> String {
    line
}

struct State {
    winds: Option<String>,
}

impl State {
    pub fn new() -> State {
        State { winds: None }
    }
}

fn accumulate(mut state: State, line: String) -> State {
    println!("Line length is {}", line.len());
    state.winds = Some(line);
    state
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct OccupiedSpace {
    x: i32,
    y: i64,
}

impl OccupiedSpace {
    pub fn new(x: i32, y: i64) -> OccupiedSpace {
        OccupiedSpace { x, y }
    }
    pub fn to_absolute(&self, x: i32, y: i64) -> OccupiedSpace {
        OccupiedSpace {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

struct Rock {
    relative_occupieds: Vec<OccupiedSpace>,
}

impl Rock {
    pub fn new(relative_occupieds: Vec<OccupiedSpace>) -> Rock {
        Rock { relative_occupieds }
    }
}

enum Direction {
    Left,
    Right,
}

const LEFT_WALL: i32 = -1;
const RIGHT_WALL: i32 = 7;
const FLOOR_HEIGHT: i64 = -1;

const DROP_LEFT: i32 = 2;
const DROP_HEIGHT: i64 = 3;

const NUM_ROCKS_1: usize = 2022;
const NUM_ROCKS_2: usize = 1000000000000;
//const NUM_ROCKS_2: usize = 2022;

const REPORT_EVERY: usize = 100000000;
const MAX_OCCUPIED_ROWS: usize = 100;

fn reduce1(state: State) -> i64 {
    reduce(&state, NUM_ROCKS_1)
}

fn reduce2(state: State) -> i64 {
    //Start with something suitably high that a repeat will have set in
    let start_num = 5000;
    let start_height = reduce(&state, start_num);

    //Now try to find where to repeat (multiples of the number of rocks)
    let rocks = generate_rocks();
    let search_increment = rocks.len();
    let mut repeat_size = search_increment;
    let repeat_height;
    loop {
        println!("Trying {}", repeat_size);
        let height1 = reduce(&state, start_num + repeat_size);
        let height2 = reduce(&state, start_num + 2 * repeat_size);
        let height3 = reduce(&state, start_num + 3 * repeat_size);
        if height1 - start_height == height2 - height1 &&
           height2 - height1 == height3 - height2 {
            println!("Found repeat: {}", repeat_size);
            repeat_height = height2 - height1;
            println!("Repeat height is: {}", repeat_height);
            break;
        }
        repeat_size += search_increment;
    }

    //Finally we can do the calc
    let remaining_to_calculate = NUM_ROCKS_2 - start_num;
    let num_repeats_needed = (remaining_to_calculate / repeat_size) - 1;
    let final_to_calculate = remaining_to_calculate - (repeat_size * num_repeats_needed);
    let final_height = reduce(&state, final_to_calculate + start_num) - start_height;
    start_height + (num_repeats_needed as i64 * repeat_height) + final_height
}

fn reduce(state: &State, num_rocks: usize) -> i64 {
    let rocks = generate_rocks();
    let mut chars = state.winds.as_ref().unwrap().chars();
    let mut latest_height: i64 = 0;

    let mut next_direction_and_reset = || {
        let mut direction = get_next_direction(&mut chars);
        let mut reset = false;
        while direction.is_none() {
            chars = state.winds.as_ref().unwrap().chars();
            reset = true;
            direction = get_next_direction(&mut chars);
        }
        (direction.unwrap(), reset)
    };

    let mut occupied_spaces: BTreeMap<i64, HashSet<i32>> = BTreeMap::new();

    for rock_num in 0..num_rocks {

        let rock_mod = rock_num % rocks.len();
        let rock = rocks.get(rock_mod).unwrap();

        let mut left = DROP_LEFT;
        let mut height = latest_height + DROP_HEIGHT;
        let mut fell = true;

        output_tower(&rock, None, left, height, latest_height, &occupied_spaces);

        while fell {
            let (direction, reset) = next_direction_and_reset();
            if reset {
                // println!("Reset at rock_num {}, mod {}, with latest_height {} after {} moves", rock_num, rock_mod, latest_height, num_rock_moves);
            }

            left = wind_push(&rock, left, height, &direction, &occupied_spaces);
            output_tower(&rock, Some(&direction), left, height, latest_height, &occupied_spaces);
            (height, fell)  = rock_falls(&rock, left, height, &occupied_spaces);
            output_tower(&rock, None, left, height, latest_height, &occupied_spaces);
            if !fell {
                for occupied in rock.relative_occupieds.iter() {
                    let absolute_occupied = occupied.to_absolute(left, height);
                    latest_height = std::cmp::max(latest_height, absolute_occupied.y + 1); //plus one as block takes up 1

                    let row = occupied_spaces.entry(absolute_occupied.y).or_insert_with(|| HashSet::new());
                    row.insert(absolute_occupied.x);
                }
                output_tower(&rock, None, left, height, latest_height, &occupied_spaces);
            }
        }

        while occupied_spaces.len() > MAX_OCCUPIED_ROWS {
            occupied_spaces.pop_first();
        }

        if rock_num > 0 && rock_num % REPORT_EVERY == 0 {
            println!("{}", rock_num);
        }
    }

    latest_height
}

///Try to push the rock in the direction, stopping if it would hit the wall, or an occupied space
/// returns the 'new' left (which will not be changed, if it wasn't possible)
fn wind_push(rock: &Rock, left: i32, height: i64, direction: &Direction, occupied_spaces: &BTreeMap<i64, HashSet<i32>>) -> i32 {
    let new_left = match direction {
        Direction::Left => left - 1,
        Direction::Right => left + 1,
    };
    //check each space occupied by the rock...  if any would not be possible, then don't move
    for space in rock.relative_occupieds.iter() {
        let absolute = space.to_absolute(new_left, height);
        //left wall or right wall
        if absolute.x <= LEFT_WALL ||absolute.x >= RIGHT_WALL {
            return left;
        }
        //is occupied?
        let occupied_row = occupied_spaces.get(&absolute.y);
        if let Some(map) = occupied_row {
            if map.contains(&absolute.x) {
                return left;
            }
        }
    }
    //all were ok - new left is good:
    new_left
}

///Try to let the rock fall - returning the new height and true if it could, otherwise the original height and false
fn rock_falls(rock: &Rock, left: i32, height: i64, occupied_spaces: &BTreeMap<i64, HashSet<i32>>) -> (i64, bool) {
    let new_height = height - 1;
    //check whether the rock is going to hit anything occupied
    for space in rock.relative_occupieds.iter() {
        let absolute = space.to_absolute(left, new_height);
        //floor
        if absolute.y <= FLOOR_HEIGHT {
            return (height, false);
        }
        //is occupied?
        let occupied_row = occupied_spaces.get(&absolute.y);
        if let Some(map) = occupied_row {
            if map.contains(&absolute.x) {
                return (height, false);
            }
        }
    }
    (new_height, true)
}


fn generate_rocks() -> Vec<Rock> {
    let rock1: Rock = Rock::new(vec![
        OccupiedSpace::new(0, 0),
        OccupiedSpace::new(1, 0),
        OccupiedSpace::new(2, 0),
        OccupiedSpace::new(3, 0),
    ]);
    let rock2: Rock = Rock::new(vec![
        OccupiedSpace::new(1, 0),
        OccupiedSpace::new(0, 1),
        OccupiedSpace::new(1, 1),
        OccupiedSpace::new(2, 1),
        OccupiedSpace::new(1, 2),
    ]);
    let rock3: Rock = Rock::new(vec![
        OccupiedSpace::new(0, 0),
        OccupiedSpace::new(1, 0),
        OccupiedSpace::new(2, 0),
        OccupiedSpace::new(2, 1),
        OccupiedSpace::new(2, 2),
    ]);
    let rock4: Rock = Rock::new(vec![
        OccupiedSpace::new(0, 0),
        OccupiedSpace::new(0, 1),
        OccupiedSpace::new(0, 2),
        OccupiedSpace::new(0, 3),
    ]);
    let rock5: Rock = Rock::new(vec![
        OccupiedSpace::new(0, 0),
        OccupiedSpace::new(1, 0),
        OccupiedSpace::new(0, 1),
        OccupiedSpace::new(1, 1),
    ]);
    vec![rock1, rock2, rock3, rock4, rock5]
}

fn get_next_direction(chars: &mut Chars) -> Option<Direction> {
    match chars.next() {
        Some('<') => Some(Direction::Left),
        Some('>') => Some(Direction::Right),
        _ => None,
    }
}

fn output_tower(_last_rock: &Rock, _direction: Option<&Direction>, _rock_left: i32, _rock_height: i64, _max_height: i64,
     _occupied_spaces: &BTreeMap<i64, HashSet<i32>>)
{
}

// fn output_tower(last_rock: &Rock, direction: Option<&Direction>, rock_left: i32, rock_height: i32, max_height: i32,
//     occupied_spaces: &HashSet<OccupiedSpace>)
// {
//     match direction {
//         Some(Direction::Left) => println!("<-- Left"),
//         Some(Direction::Right) => println!("Right -->"),
//         None => {},
//     }
//     for height in (0..max_height + DROP_HEIGHT + 3).rev() {
//         'inner: for x in 0..RIGHT_WALL {
//             let space = OccupiedSpace::new(x, height);
//             //is this occupied by the rock?
//             for rock_relative in last_rock.relative_occupieds.iter() {
//                 let absolute = rock_relative.to_absolute(rock_left, rock_height);
//                 if space == absolute {
//                     print!("@");
//                     continue 'inner;
//                 }
//             }
//             //is this occupied
//             if occupied_spaces.contains(&space) {
//                 print!("#");
//                 continue 'inner;
//             }
//             //not occupied
//             print!(".");
//         }
//         println!("");
//     }
//     println!("");
// }
