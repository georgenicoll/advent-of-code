use std::fmt::Display;
use std::cmp::Ordering;

use crate::utils;
const FILE_NAME: &str = "13/input.txt";
//const FILE_NAME: &str = "13/test_input.txt";

pub fn _13a() -> Result<usize, std::io::Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new(),
        accumulate1,
        reduce1
    )
}

pub fn _13b() -> Result<usize, std::io::Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new(),
        accumulate2,
        reduce2
    )
}

type Value = i16;

#[derive(Debug)]
enum Item{
    Val{ val: Value },
    List{ vec: Vec<Item> },
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Val{ val } => write!(f, "{}", val),
            Item::List{ vec } => {
                write!(f, "[").expect("write failed");
                for (index, item) in vec.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{}", item).expect("write failed");
                    } else {
                        write!(f, ",{}", item).expect("write failed");
                    }
                }
                write!(f, "]")
            },
        }
    }
}

fn parse_line(line: String) -> Option<Item> {
    let mut result = None;
    let mut stack: Vec<Vec<Item>> = Vec::new();
    let mut chars: String = String::new();
    for c in line.chars() {
        match c {
            '[' => stack.push(Vec::new()),
            ']' => {
                add_value(&mut chars, stack.last_mut().unwrap());
                let vec = stack.pop().unwrap();
                let item = Item::List { vec };
                if stack.is_empty() {
                    result = Some(item);
                } else {
                    stack.last_mut().unwrap().push(item);
                }
            },
            ',' => {
                add_value(&mut chars, stack.last_mut().unwrap());
            },
            _ => chars.push(c),
        }
    }
    result
}

fn add_value(chars: &mut String, vec: &mut Vec<Item>) {
    if !chars.is_empty() {
        let val: Value = chars.parse().unwrap();
        chars.clear();
        let item = Item::Val { val };
        vec.push(item);
    }
}

struct State {
    items: Vec<Item>,
    current_index: usize,
    sum_correct_order_indices: usize,
}

impl State {
    fn new() -> State {
        State {
            items: Vec::with_capacity(2),
            current_index: 0,
            sum_correct_order_indices: 0,
        }
    }
}

fn progress_next(state: &mut State) {
    state.current_index += 1;
    if in_right_order(&state.items) {
        state.sum_correct_order_indices += state.current_index;
    }
    state.items.clear();
}

fn in_right_order(items: &Vec<Item>) -> bool {
    if items.len() != 2 {
        panic!("Unexpected items length: {}", items.len());
    }
    let left = items.get(0).unwrap();
    let right = items.get(1).unwrap();
    compare_left_and_right(left, right) < 0
}

///Compare the items, returns an integer where:
/// result < 0, left is less than right
/// result = 0, left is equal to right
/// result > 0, left is greater than right
fn compare_left_and_right(left: &Item, right: &Item) -> Value {
    match (left, right) {
        (Item::Val { val: left_val }, Item::Val{ val: right_val }) => {
            left_val - right_val
        },
        (Item::List { vec: _vec }, Item::Val { val }) => {
            compare_left_and_right(left, &Item::List { vec: vec!(Item::Val{ val: *val }) })
        },
        (Item::Val { val }, Item::List { vec: _vec }) => {
            compare_left_and_right(&Item::List { vec: vec!(Item::Val{ val: *val }) }, right)
        },
        (Item::List { vec: left_vec}, Item::List { vec: right_vec}) => {
            for index in 0..left_vec.len() {
                let left_item = left_vec.get(index).unwrap();
                let right_item = right_vec.get(index);
                if right_item.is_none() {
                    return 1; //left is bigger than right - it has more elements
                }
                let comparison_result = compare_left_and_right(left_item, right_item.unwrap());
                if comparison_result != 0 {
                    return comparison_result;
                }
            }
            //if we get here, all items were equal... if there are more on the right side then
            //it's in the correct order, or_else, we're equal
            if right_vec.len() > left_vec.len() {
                return -1;
            }
            0
        },
    }
}

fn accumulate1(mut state: State, opt_item: Option<Item>) -> State {
    match opt_item {
        Some(item) => {
            state.items.push(item);
        },
        None => {
            progress_next(&mut state);
        },
    }
    state
}

fn reduce1(mut state: State) -> usize {
    if !state.items.is_empty() {
        progress_next(&mut state);
    }
    state.sum_correct_order_indices
}

fn accumulate2(mut state: State, opt_item: Option<Item>) -> State {
    match opt_item {
        Some(item) => {
            state.items.push(item);
        },
        None => {}
    }
    state
}

fn reduce2(mut state: State) -> usize {
    fn divider1() -> Item {
        Item::List{ vec: vec!(Item::List{ vec: vec!(Item::Val { val: 2 }) }) }
    }
    fn divider2() -> Item {
        Item::List{ vec: vec!(Item::List{ vec: vec!(Item::Val { val: 6 }) }) }
    }

    state.items.push(divider1());
    state.items.push(divider2());

    state.items.sort_by(|left, right| {
        let comparison = compare_left_and_right(left, right);
        if comparison < 0 {
            return Ordering::Less;
        };
        if comparison == 0 {
            return Ordering::Equal;
        }
        Ordering::Greater
    });

    let divider1 = divider1();
    let divider2 = divider2();
    let mut first_index: Option<usize> = None;
    let mut second_index: Option<usize> = None;
    for (index, item) in state.items.iter().enumerate() {
        let one_based_index = index + 1;
        if first_index.is_none() && compare_left_and_right(&divider1, item) == 0 {
            first_index = Some(one_based_index);
            // println!("**{}** {}", one_based_index, item);
        } else if second_index.is_none() && compare_left_and_right(&divider2, item) == 0 {
            second_index = Some(one_based_index);
            // println!("**{}** {}", one_based_index, item);
        } else {
            // println!("{}", item);
        }
    }
    first_index.unwrap() * second_index.unwrap()
}