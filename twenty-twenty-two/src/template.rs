use crate::utils;

const FILE_NAME: &str = "template/input.txt";

pub fn _template_a() -> Result<u32, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce)
}

pub fn _template_b() -> Result<u32, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce)
}

fn parse_line(line: String) -> String {
    line
}

struct State {
}

impl State {
    pub fn new() -> State {
        State {
        }
    }
}

fn accumulate(state: State, _line: String) -> State {
    state
}

fn reduce(_state: State) -> u32 {
    0
}
