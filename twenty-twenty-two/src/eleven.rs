use core::fmt;
use std::io::Error;
use std::collections::{VecDeque, BTreeMap};

use substring::Substring;

use crate::utils::process_file;

const FILENAME: &str = "11/input.txt";

// type WorryLevel = u128;
// type WorryLevel = u64;
type WorryLevel = usize;

pub fn _11a() -> Result<WorryLevel, Error> {
    process_file(
        FILENAME,
        parse,
        State::new(),
        accumulate,
        calculate_monkey_business1,
    )
}

pub fn _11b() -> Result<WorryLevel, Error> {
    process_file(
        FILENAME,
        parse,
        State::new(),
        accumulate,
        calculate_monkey_business2,
    )
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply{ value: WorryLevel },
    Add{ value: WorryLevel },
    Square,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Multiply { value } => write!(f, "Multiply by {}", value),
            Self::Add { value } => write!(f, "Add {}", value),
            Self::Square => write!(f, "Square"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    item_worry_levels: VecDeque<WorryLevel>,
    operation: Operation,
    test_divisor: WorryLevel,
    true_monkey_id: usize,
    false_monkey_id: usize,
    num_inspections_made: WorryLevel,
}

impl Monkey {
    pub fn new(id: usize, item_worry_levels: VecDeque<WorryLevel>, operation: Operation,
               test_divisor: WorryLevel, true_monkey_id: usize, false_monkey_id: usize)
               -> Monkey {
        Monkey {
            id,
            item_worry_levels,
            operation,
            test_divisor,
            true_monkey_id,
            false_monkey_id,
            num_inspections_made: 0,
        }
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Monkey(id: {}, operation: {}, divisor: {}, \
             true_monkey: {}, false_monkey: {}, num_inspections_made: {}, worry_levels: {:?})",
            self.id, self.operation, self.test_divisor,
            self.true_monkey_id, self.false_monkey_id, self.num_inspections_made,
            self.item_worry_levels
        )
    }
}

struct MonkeyUnderConstruction {
    id: usize,
    item_worry_levels: Option<VecDeque<WorryLevel>>,
    operation: Option<Operation>,
    test_divisor: Option<WorryLevel>,
    true_monkey_id: Option<usize>,
    false_monkey_id: Option<usize>,
}

impl MonkeyUnderConstruction {
    pub fn new(id: usize) -> MonkeyUnderConstruction {
        MonkeyUnderConstruction {
            id: id,
            item_worry_levels: None,
            operation: None,
            test_divisor: None,
            true_monkey_id: None,
            false_monkey_id: None,
        }
    }

    pub fn construct_monkey(self) -> Monkey {
        Monkey::new(
            self.id,
            self.item_worry_levels.unwrap(),
            self.operation.unwrap(),
            self.test_divisor.unwrap(),
            self.true_monkey_id.unwrap(),
            self.false_monkey_id.unwrap()
        )
    }
}

struct State {
    monkey_under_construction: Option<MonkeyUnderConstruction>,
    monkeys: BTreeMap<usize, Monkey>,
}

impl State {
    pub fn new() -> State {
        State {
            monkey_under_construction: None,
            monkeys: BTreeMap::new(),
        }
    }
}

enum Command {
    NewMonkey{ id: usize },
    StartingItems{ item_worry_levels: Vec<WorryLevel> },
    Operation{ op: Operation },
    Test{ divisor: WorryLevel },
    TrueMonkey{ id: usize },
    FalseMonkey{ id: usize },
    MakeMonkey,
}

fn parse(line: String) -> Command {
    let first_five = line.substring(0, 5);
    match first_five {
        "Monke" => parse_monkey(line),
        "  Sta" => parse_starting_items(line),
        "  Ope" => parse_operation(line),
        "  Tes" => parse_test(line),
        "    I" => parse_true_or_false_monkey(line),
        _ => Command::MakeMonkey,
    }
}

fn parse_monkey(line: String) -> Command {
    //Monkey id:
    let id_string = line.substring(7, line.len() - 1);
    Command::NewMonkey {
        id: id_string.parse().unwrap()
    }
}

fn parse_starting_items(line: String) -> Command {
    //  Starting items: item, item, item, item
    let worry_level_strings = line.substring(18, line.len());
    let mut item_worry_levels: Vec<WorryLevel> = Vec::new();
    for worry_level in worry_level_strings.split(", ").into_iter() {
        item_worry_levels.push(worry_level.parse().unwrap());
    }
    Command::StartingItems { item_worry_levels }
}

fn parse_operation(line: String) -> Command {
    //  Operation: new = old * value
    //or
    //  Operation: new = old + value
    let op_string = line.substring(23, 24);
    let value_string = line.substring(25, line.len());
    match (op_string, value_string) {
        ("*", "old") => Command::Operation { op: Operation::Square },
        ("*", _) => Command::Operation{ op: Operation::Multiply { value: value_string.parse().unwrap() }},
        ("+", _) => Command::Operation{ op: Operation::Add { value: value_string.parse().unwrap() }},
        _ => panic!("Unrecognised Operation: {}", line),
    }
}

fn parse_test(line: String) -> Command {
    //  Test: divisible by value
    let value_string = line.substring(21, line.len());
    Command::Test { divisor: value_string.parse().unwrap() }
}

fn parse_true_or_false_monkey(line: String) -> Command {
    //    If true: throw to monkey 3
    //    If false: throw to monkey 0
    let if_string = line.substring(7, line.len());
    let splitted: Vec<&str> = if_string.split(|c| c == ' ' || c == ':').collect();
    let true_or_false = splitted.first().unwrap();
    let id_string = splitted.last().unwrap();
    match *true_or_false {
        "true" => Command::TrueMonkey { id: id_string.parse().unwrap() },
        "false" => Command::FalseMonkey { id: id_string.parse().unwrap() },
        _ => panic!("Unrecognised If line: {}", line),
    }
}

fn accumulate(mut state: State, command: Command) -> State {
    match command {
        Command::NewMonkey { id } => {
            //println!("New Monkey under construction: {}", id);
            state.monkey_under_construction = Some(MonkeyUnderConstruction::new(id));
            state
        },
        Command::StartingItems { item_worry_levels } => {
            state.monkey_under_construction.as_mut().unwrap().item_worry_levels = Some(VecDeque::from(item_worry_levels));
            state
        },
        Command::Operation { op } => {
            state.monkey_under_construction.as_mut().unwrap().operation = Some(op);
            state
        }
        Command::Test { divisor } => {
            state.monkey_under_construction.as_mut().unwrap().test_divisor = Some(divisor);
            state
        },
        Command::TrueMonkey { id } => {
            state.monkey_under_construction.as_mut().unwrap().true_monkey_id = Some(id);
            state
        },
        Command::FalseMonkey { id } => {
            state.monkey_under_construction.as_mut().unwrap().false_monkey_id = Some(id);
            state
        },
        Command::MakeMonkey => {
            let under_construction = state.monkey_under_construction.take().unwrap();
            let monkey = under_construction.construct_monkey();
            //println!("{}", monkey);
            state.monkeys.insert(monkey.id, monkey);
            state
        }
    }
}

fn calculate_monkey_business1(state: State) -> WorryLevel {
    calculate_monkey_business(state, 20, 3)
}

fn calculate_monkey_business2(state: State) -> WorryLevel {
    calculate_monkey_business(state, 10000, 1)
}

fn calculate_monkey_business(mut state: State, num_rounds: usize, worry_level_post_inspection_divisor: WorryLevel) -> WorryLevel {
    //if there is a monkey still under construction, ensure we finish its construction now
    state = if state.monkey_under_construction.is_some() {
        accumulate(state, Command::MakeMonkey)
    } else {
        state
    };

    //work out what we can use to mod any value we keep - we can take the mod as each of our divisors multiplied together.
    //and still keep the true/false modulo semantics
    let mod_worry_level: WorryLevel = state.monkeys.values()
        .map(|monkey| monkey.test_divisor)
        .fold(1, |acc, divisor| acc * divisor);

    //now run the monkey business...
    state = perform_rounds(state, num_rounds, worry_level_post_inspection_divisor, mod_worry_level);

    //... and calculate the monkey business after the rounds
    let mut collections: Vec<_> = state.monkeys.values().map(|monkey| monkey.num_inspections_made).collect();
    collections.sort();
    collections.reverse();
    collections.truncate(2);
    collections.iter().fold(1, |acc, item| acc * item)
}

fn perform_rounds(mut state: State,
                  num_rounds: usize,
                  worry_level_post_inspection_divisor: WorryLevel,
                  mod_worry_level: WorryLevel) -> State {
    let monkey_ids: Vec<_> = state.monkeys.keys().cloned().collect();
    for _round in 0..num_rounds {
        //println!("=== Round {} ===", _round);
        for monkey_id in monkey_ids.iter() {
            //extract the worry levels from the source monkey (and update)
            let monkey = state.monkeys.get_mut(monkey_id).unwrap();
            monkey.num_inspections_made += monkey.item_worry_levels.len() as WorryLevel;
            let worry_levels: Vec<_> = monkey.item_worry_levels.drain(..).collect();
            let monkey_copy = Monkey {
                item_worry_levels: VecDeque::with_capacity(0),
                ..*monkey
            };
            //update and push worry levels to new monkeys
            for worry_level in worry_levels {
                let worry_level_during_inspection = match monkey_copy.operation {
                    Operation::Multiply { value } => worry_level * value,
                    Operation::Add { value } => worry_level + value,
                    Operation::Square => worry_level * worry_level,
                };
                let worry_level_after_inspection = worry_level_during_inspection / worry_level_post_inspection_divisor;
                let worry_level_after_inspection = worry_level_after_inspection % mod_worry_level;
                let destination_monkey_id = if worry_level_after_inspection % monkey_copy.test_divisor == 0 {
                    monkey_copy.true_monkey_id
                } else {
                    monkey_copy.false_monkey_id
                };
                //println!("Monkey {} pushes {} ({} {} / 3) to monkey {}",
                //    monkey_id, worry_level_after_inspection, worry_level, monkey_copy.operation, destination_monkey_id);
                let dest_monkey = state.monkeys.get_mut(&destination_monkey_id).unwrap();
                dest_monkey.item_worry_levels.push_back(worry_level_after_inspection);
            }
        }
    }
    state
}
