use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::rc::Rc;

use lazy_static::__Deref;

use crate::utils;

// const FILE_NAME: &str = "16/input.txt";
const FILE_NAME: &str = "16/test_input.txt";

pub fn _16a() -> Result<u32, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce)
}

pub fn _16b() -> Result<u32, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce)
}

type ValveID = String;
type FlowRate = usize;

#[derive(Debug)]
struct Valve {
    id: Rc<ValveID>,
    flow_rate: FlowRate,
    tunnels_to: Vec<Rc<ValveID>>,
}

impl fmt::Display for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}] -> ", self.id, self.flow_rate).unwrap();
        utils::output_into_iter(f, "|", &self.tunnels_to);
        write!(f, "")
    }
}

fn parse_line(line: String) -> Valve {
    //Valve VB has flow rate=20; tunnels lead to valves UU, EY, SG, ZB
    let mut chars = line.chars();
    utils::skip(&mut chars, 6); // "Valve "
    let id = utils::parse_next_string(&mut chars);
    utils::skip(&mut chars, 14); //"has flow rate="
    let flow_rate: FlowRate = utils::parse_next_number(&mut chars).unwrap();
    utils::skip(&mut chars, 22); //" tunnels lead to valve"
                                 //Check for plural and consume a space (otherwise the space was just consumed)
    if let Some('s') = chars.next() {
        utils::skip(&mut chars, 1);
    }
    let mut tunnels_to: Vec<Rc<ValveID>> = Vec::new();
    loop {
        let tunnel_to = utils::parse_next_string(&mut chars);
        if tunnel_to.len() == 0 {
            break;
        }
        tunnels_to.push(Rc::new(tunnel_to));
        //consume space (if following the comma)
        utils::skip(&mut chars, 1);
    }
    Valve {
        id: Rc::new(id),
        flow_rate,
        tunnels_to,
    }
}

struct State {
    valves: HashMap<Rc<ValveID>, Valve>,
}

impl State {
    pub fn new() -> State {
        State {
            valves: HashMap::new(),
        }
    }
}

fn accumulate(mut state: State, valve: Valve) -> State {
    println!("{}", valve);
    state.valves.insert(valve.id.clone(), valve);
    state
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Navigation {
    from: Rc<ValveID>,
    to: Rc<ValveID>,
}

impl Navigation {
    pub fn new(from: Rc<ValveID>, to: Rc<ValveID>) -> Navigation {
        Navigation { from, to }
    }
}

type TotalPressure = u32;

#[derive(Debug)]
struct Visit {
    id: Rc<ValveID>,
    time_cost_to_reach: usize,
    released_flow_rate: FlowRate,
    total_pressure_released: TotalPressure,
    opened_valves: Rc<HashSet<Rc<ValveID>>>,
    navigations: HashSet<Navigation>,
    visit_order: Vec<Rc<String>>,
}

impl Visit {
    pub fn new(
        id: Rc<ValveID>,
        time_cost_to_reach: usize,
        released_flow_rate: FlowRate,
        total_pressure_released: TotalPressure,
        opened_valves: Rc<HashSet<Rc<ValveID>>>,
        navigations: HashSet<Navigation>,
        visit_order: Vec<Rc<String>>,
    ) -> Visit {
        Visit {
            id,
            time_cost_to_reach,
            released_flow_rate,
            total_pressure_released,
            opened_valves,
            navigations,
            visit_order,
        }
    }
}

const MAX_TIME: usize = 30;

fn reduce(state: State) -> u32 {
    // utils::output_into_iter_io(io::stdout(), ",", state.valves.values().into_iter());

    //calculate the maximum number of valves that it makes sense to open
    let max_valves_to_open: usize = state
        .valves
        .values()
        .filter(|valve| valve.flow_rate > 0)
        .count();

    //dfs - prime the first node
    let mut visits: VecDeque<Visit> = VecDeque::new();
    {
        let start_valve = state.valves.get(&String::from("AA")).unwrap();
        let new_opened_valves: HashSet<Rc<ValveID>> = HashSet::new();
        visits.push_back(Visit::new(
            Rc::clone(&start_valve.id),
            0,
            0,
            0,
            Rc::new(new_opened_valves),
            HashSet::new(),
            Vec::new(),
        ));
    }

    let mut max_total_pressure_released: u32 = 0;
    let mut visited_order: Vec<Rc<String>> = Vec::new();

    while !visits.is_empty() {
        let visit = visits.pop_back().unwrap();

        // println!("{:?}", visit);

        let valve = match state.valves.get(&visit.id) {
            Some(v) => v,
            None => panic!("Failed to find {}", visit.id),
        };

        //did we get to MAX_TIME?
        if visit.time_cost_to_reach >= MAX_TIME {
            if max_total_pressure_released < visit.total_pressure_released {
                max_total_pressure_released = visit.total_pressure_released;
                visited_order = visit.visit_order;
            }
            continue;
        }

        //did we visit all open valves? (in which case we can't increase the flow rate further)
        if visit.opened_valves.len() >= max_valves_to_open {
            let final_total_pressure_released = visit.total_pressure_released
                + (visit.released_flow_rate as TotalPressure
                    * (MAX_TIME - visit.time_cost_to_reach) as TotalPressure);
            if max_total_pressure_released < final_total_pressure_released {
                max_total_pressure_released = final_total_pressure_released;
                visited_order = visit.visit_order;
            }
            continue;
        }

        let mut added_visit = false;
        //re-visit with the valve opened??
        if valve.flow_rate > 0 && !visit.opened_valves.contains(&valve.id) {
            let mut new_opened_valves = visit.opened_valves.deref().clone();
            new_opened_valves.insert(Rc::clone(&valve.id));
            let mut new_visited_order = visit.visit_order.clone();
            new_visited_order.push(Rc::clone(&valve.id));
            let new_visit = Visit::new(
                Rc::clone(&visit.id),
                visit.time_cost_to_reach + 1,
                visit.released_flow_rate + valve.flow_rate,
                visit.total_pressure_released + visit.released_flow_rate as TotalPressure,
                Rc::new(new_opened_valves),
                visit.navigations,
                new_visited_order,
            );
            visits.push_back(new_visit);
            added_visit = true;
        } else {
            //navigate to any we didn't navigate to yet in this search
            for to in valve.tunnels_to.iter() {
                let navigation = Navigation::new(Rc::clone(&visit.id), Rc::clone(to));
                if visit.navigations.contains(&navigation) {
                    continue;
                }
                let mut new_navigations = visit.navigations.clone();
                new_navigations.insert(navigation);
                let mut new_visited_order = visit.visit_order.clone();
                new_visited_order.push(Rc::clone(&to));
                let new_visit = Visit::new(
                    Rc::clone(to),
                    visit.time_cost_to_reach + 1,
                    visit.released_flow_rate,
                    visit.total_pressure_released + visit.released_flow_rate as TotalPressure,
                    Rc::clone(&visit.opened_valves),
                    new_navigations,
                    new_visited_order,
                );
                visits.push_back(new_visit);
                added_visit = true
            }
        }

        if !added_visit {
            //this path has come to an end...  calculate what the final total would be
            let final_total_pressure_released = visit.total_pressure_released
                + (visit.released_flow_rate as TotalPressure
                    * (MAX_TIME - visit.time_cost_to_reach) as TotalPressure);
            if max_total_pressure_released < final_total_pressure_released {
                max_total_pressure_released = final_total_pressure_released;
                visited_order = visit.visit_order;
            }
            continue;
        }
    }

    utils::output_into_iter_io(io::stdout(), ",", visited_order.into_iter());
    println!("");
    max_total_pressure_released
}
