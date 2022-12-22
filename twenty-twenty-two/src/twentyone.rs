use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::rc::Rc;

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils;

type MonkeyID = String;
type Val = f64;
// type Val = i64;
const ROOT: &str = "root";
const HUMAN: &str = "humn";

const FILE_NAME: &str = "21/input.txt";
// const FILE_NAME: &str = "21/test_input.txt";

pub fn _21a() -> Result<Val, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce1)
}

pub fn _21b() -> Result<Val, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce2)
}

enum Operation {
    Plus,
    Minus,
    Times,
    DividedBy,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operation::Plus => "+",
            Operation::Minus => "-",
            Operation::Times => "*",
            Operation::DividedBy => "/",
        };
        write!(f, "{}", op)
    }
}

enum MonkeyCalc {
    Value {
        value: Val,
    },
    Op {
        ref1: Rc<MonkeyID>,
        op: Operation,
        ref2: Rc<MonkeyID>,
    },
}

impl Display for MonkeyCalc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonkeyCalc::Value { value } => write!(f, "{}", value),
            MonkeyCalc::Op { ref1, op, ref2 } => write!(f, "{} {} {}", ref1, op, ref2),
        }
    }
}

struct Monkey {
    id: Rc<MonkeyID>,
    calc: MonkeyCalc,
    is_root: bool,
}

impl Monkey {
    pub fn new_value(id: MonkeyID, value: Val) -> Monkey {
        let is_root = Monkey::is_root(&id);
        Monkey {
            id: Rc::new(id),
            calc: MonkeyCalc::Value { value },
            is_root,
        }
    }
    pub fn new_op(id: MonkeyID, ref1: MonkeyID, op: Operation, ref2: MonkeyID) -> Monkey {
        let is_root = Monkey::is_root(&id);
        Monkey {
            id: Rc::new(id),
            calc: MonkeyCalc::Op {
                ref1: Rc::new(ref1),
                op,
                ref2: Rc::new(ref2),
            },
            is_root,
        }
    }
    fn is_root(id: &MonkeyID) -> bool {
        id == ROOT
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root_string = if self.is_root { "*" } else { "" };
        write!(f, "{}{}: {}", self.id, root_string, self.calc)
    }
}

fn parse_line(line: String) -> Monkey {
    lazy_static! {
        static ref VAL: Regex = Regex::new(r"^(\w+): (\d+)$").unwrap();
        static ref OP: Regex = Regex::new(r"^(\w+): (\w+) (\+|\-|\*|/) (\w+)$").unwrap();
    }
    None.or_else(|| {
        VAL.captures(&line).map(|caps| {
            let id = String::from(caps.get(1).unwrap().as_str());
            let value: Val = caps.get(2).unwrap().as_str().parse().unwrap();
            Monkey::new_value(id, value)
        })
    })
    .or_else(|| {
        OP.captures(&line).map(|caps| {
            let id = String::from(caps.get(1).unwrap().as_str());
            let ref1 = String::from(caps.get(2).unwrap().as_str());
            let op = parse_operation(caps.get(3).unwrap().as_str()).expect(line.as_str());
            let ref2 = String::from(caps.get(4).unwrap().as_str());
            Monkey::new_op(id, ref1, op, ref2)
        })
    })
    .expect(line.as_str())
}

fn parse_operation(op: &str) -> Option<Operation> {
    match op {
        "+" => Some(Operation::Plus),
        "-" => Some(Operation::Minus),
        "*" => Some(Operation::Times),
        "/" => Some(Operation::DividedBy),
        _ => None,
    }
}

///monkey_back_refs contains a map of refered to referer, i.e.
/// this is a dependency, key is depended by value
struct State {
    monkeys: Vec<Rc<Monkey>>,
    monkeys_by_id: HashMap<Rc<MonkeyID>, Rc<Monkey>>,
    monkey_back_refs: HashMap<Rc<MonkeyID>, Rc<MonkeyID>>,
    monkey_values: HashMap<Rc<MonkeyID>, Val>,
}

impl State {
    pub fn new() -> State {
        State {
            monkeys: Vec::new(),
            monkeys_by_id: HashMap::new(),
            monkey_back_refs: HashMap::new(),
            monkey_values: HashMap::new(),
        }
    }
}

fn accumulate(mut state: State, monkey: Monkey) -> State {
    // println!("{}", monkey);
    if let Some(value) = do_monkey_calc(&state.monkey_values, &monkey.calc) {
        state.monkey_values.insert(Rc::clone(&monkey.id), value);
    }
    match &monkey.calc {
        MonkeyCalc::Op {
            ref1,
            op: _op,
            ref2,
        } => {
            state
                .monkey_back_refs
                .insert(Rc::clone(ref1), Rc::clone(&monkey.id));
            state
                .monkey_back_refs
                .insert(Rc::clone(ref2), Rc::clone(&monkey.id));
        }
        _ => {}
    }
    let monkey_rc = Rc::new(monkey);
    state
        .monkeys_by_id
        .insert(Rc::clone(&monkey_rc.id), Rc::clone(&monkey_rc));
    state.monkeys.push(Rc::clone(&monkey_rc));
    state
}

fn reduce1(mut state: State) -> Val {
    reduce(&mut state)
}

fn reduce(state: &mut State) -> Val {
    //Output
    // println!();
    //repeatedly loop until we can calculate the root
    let mut num_loops = 0;
    let root_value: Option<Val>;
    'outer: loop {
        //Output
        // utils::output_into_iter_io(std::io::stdout(), "\n", &mut state.monkeys.iter());
        // println!();

        num_loops += 1;
        'inner: for monkey in state.monkeys.iter_mut() {
            if state.monkey_values.get(&monkey.id).is_some() {
                continue 'inner;
            }
            let monkey_value = do_monkey_calc(&state.monkey_values, &monkey.calc);
            if let Some(monkey_value) = monkey_value {
                state
                    .monkey_values
                    .insert(Rc::clone(&monkey.id), monkey_value);
                if monkey.is_root {
                    root_value = Some(monkey_value);
                    break 'outer;
                }
            }
        }
    }

    //Output
    // println!();
    // utils::output_into_iter_io(std::io::stdout(), "\n", &mut state.monkeys.iter());
    // println!();

    println!("Completed in {} loops", num_loops);
    root_value.unwrap()
}

fn do_monkey_calc(monkey_values: &HashMap<Rc<MonkeyID>, Val>, calc: &MonkeyCalc) -> Option<Val> {
    match calc {
        MonkeyCalc::Op { ref1, op, ref2 } => {
            let opt_value1 = monkey_values.get(ref1);
            let value1 = match opt_value1 {
                Some(value) => value,
                None => return None,
            };
            let opt_value2 = monkey_values.get(ref2);
            let value2 = match opt_value2 {
                Some(value) => value,
                None => return None,
            };
            Some(do_calc(value1, op, value2))
        }
        MonkeyCalc::Value { value } => Some(*value),
    }
}

fn do_calc(value1: &Val, op: &Operation, value2: &Val) -> Val {
    match op {
        Operation::Plus => value1 + value2,
        Operation::Minus => value1 - value2,
        Operation::Times => value1 * value2,
        Operation::DividedBy => value1 / value2,
    }
}

enum Side {
    Left,
    Right,
}

enum UndoOperation {
    TopMinusBottom,
    BottomMinusTop,
    Plus,
    TopDividedByBottom,
    Multiply,
    BottomDividedByTop,
}

fn reduce2(mut state: State) -> Val {
    //reduce to calculate monkey values to root...
    reduce(&mut state);

    //deduce the path to humn
    let root: Rc<MonkeyID> = Rc::new(String::from(ROOT));
    let human: Rc<MonkeyID> = Rc::new(String::from(HUMAN));

    let monkeys_in_path = get_monkeys_in_root_to_human(&state, &root, &human);

    //Starting at root, traverse the graph downwards, correcting numbers as we go until we reach human
    //then report the number that human should be.
    let root_monkey = state.monkeys_by_id.get(&root).unwrap();
    let (mut current, mut expected_value) = match &root_monkey.calc {
        MonkeyCalc::Op {
            ref1,
            op: _op,
            ref2,
        } => {
            //decide which side to traverse
            if monkeys_in_path.contains(ref1) {
                let expected_value = state.monkey_values.get(ref2).unwrap();
                (Rc::clone(&ref1), *expected_value)
            } else {
                let expected_value = state.monkey_values.get(ref1).unwrap();
                (Rc::clone(&ref2), *expected_value)
            }
        }
        MonkeyCalc::Value { value: _value } => panic!("Not expecting route to be a value"),
    };

    let correct_human_value: Option<Val>;
    loop {
        let current_monkey = state.monkeys_by_id.get(&current).unwrap();
        match &current_monkey.calc {
            MonkeyCalc::Op { ref1, op, ref2 } => {
                //Decide which side to traverse
                let (next, undo_op, correct_value) = if monkeys_in_path.contains(ref1) {
                    let correct_value = state.monkey_values.get(ref2).unwrap();
                    (
                        Rc::clone(&ref1),
                        deduce_undo_op(&op, Side::Left),
                        *correct_value,
                    )
                } else {
                    let correct_value = state.monkey_values.get(ref1).unwrap();
                    (
                        Rc::clone(&ref2),
                        deduce_undo_op(&op, Side::Right),
                        *correct_value,
                    )
                };
                expected_value = undo(undo_op, expected_value, correct_value);
                current = next;
            }
            MonkeyCalc::Value { value: _value } => {
                //this is the final value - check we got to the human 'node'
                if current != human {
                    panic!("Last was not human");
                }
                correct_human_value = Some(expected_value);
                break;
            }
        }
    }
    correct_human_value.unwrap()
}

fn get_monkeys_in_root_to_human(
    state: &State,
    root: &Rc<MonkeyID>,
    human: &Rc<MonkeyID>,
) -> HashSet<Rc<MonkeyID>> {
    let mut current = Rc::clone(&human);
    let mut monkeys_in_path: HashSet<Rc<MonkeyID>> = HashSet::new();
    monkeys_in_path.insert(Rc::clone(&current));
    print!("{}", current);
    while current != *root {
        let next = state.monkey_back_refs.get(&current).unwrap();
        print!(" -> {}", next);
        monkeys_in_path.insert(Rc::clone(next));
        current = Rc::clone(next);
    }
    println!("");

    monkeys_in_path
}

fn deduce_undo_op(op: &Operation, bad_side: Side) -> UndoOperation {
    match (op, bad_side) {
        (Operation::Plus, _) => UndoOperation::TopMinusBottom,
        (Operation::Minus, Side::Left) => UndoOperation::Plus,
        (Operation::Minus, Side::Right) => UndoOperation::BottomMinusTop,
        (Operation::Times, _) => UndoOperation::TopDividedByBottom,
        (Operation::DividedBy, Side::Left) => UndoOperation::Multiply,
        (Operation::DividedBy, Side::Right) => UndoOperation::BottomDividedByTop,
    }
}

fn undo(undo_op: UndoOperation, top: Val, bottom: Val) -> Val {
    match undo_op {
        UndoOperation::TopMinusBottom => top - bottom,
        UndoOperation::BottomMinusTop => bottom - top,
        UndoOperation::Plus => top + bottom,
        UndoOperation::TopDividedByBottom => top / bottom,
        UndoOperation::Multiply => top * bottom,
        UndoOperation::BottomDividedByTop => bottom / top,
    }
}
