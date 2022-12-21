//FIXME: Yuckie stuff for handling multiple ids
use core::fmt;
use std::cmp::{self, Ordering};
use std::collections::{HashMap, HashSet};
use std::io;
use std::rc::Rc;

use lazy_static::__Deref;

use crate::utils;

type ValveID = String;
type FlowRate = usize;
type TotalPressure = usize;

const FILE_NAME: &str = "16/input.txt";
//const FILE_NAME: &str = "16/test_input.txt";

pub fn _16a() -> Result<TotalPressure, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce1)
}

pub fn _16b() -> Result<TotalPressure, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce2)
}

#[derive(Debug)]
struct Valve {
    id: Rc<ValveID>,
    flow_rate: FlowRate,
    tunnels_to: Vec<Rc<ValveID>>,
}

impl fmt::Display for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}] -> ", self.id, self.flow_rate).unwrap();
        utils::output_into_iter(f, "|", &mut self.tunnels_to.iter());
        write!(f, "")
    }
}

const VALVE: &str = "Valve ";
const HAS_FLOW_RATE: &str = "has flow rate=";
const TUNNELS_LEAD_TO_VALVE: &str = " tunnels lead to valve";

fn parse_line(line: String) -> Valve {
    //Valve VB has flow rate=20; tunnels lead to valves UU, EY, SG, ZB
    let mut chars = line.chars();
    utils::skip(&mut chars, VALVE.len());
    let id = utils::parse_next_string(&mut chars);
    utils::skip(&mut chars, HAS_FLOW_RATE.len());
    let flow_rate: FlowRate = utils::parse_next_number(&mut chars).unwrap();
    utils::skip(&mut chars, TUNNELS_LEAD_TO_VALVE.len());
    //Check for plural and consume a space (otherwise the space was
    //just consumed)
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

#[derive(Debug)]
struct Visit {
    ids: Vec<Rc<ValveID>>,
    time_cost_to_reach: usize,
    released_flow_rate: FlowRate,
    total_pressure_released: TotalPressure,
    opened_valves: Rc<HashSet<Rc<ValveID>>>,
}

impl Visit {
    pub fn new(
        ids: Vec<Rc<ValveID>>,
        time_cost_to_reach: usize,
        released_flow_rate: FlowRate,
        total_pressure_released: TotalPressure,
        opened_valves: Rc<HashSet<Rc<ValveID>>>,
    ) -> Visit {
        Visit {
            ids,
            time_cost_to_reach,
            released_flow_rate,
            total_pressure_released,
            opened_valves,
        }
    }
}

const MAX_TIME_1: usize = 30;
const MAX_TO_KEEP1: usize = 10000;
const MAX_TIME_2: usize = 26;
const MAX_TO_KEEP2: usize = 20000;

fn reduce1(state: State) -> TotalPressure {
    reduce(state, 1, MAX_TIME_1, MAX_TO_KEEP1)
}

fn reduce2(state: State) -> TotalPressure {
    reduce(state, 2, MAX_TIME_2, MAX_TO_KEEP2)
}

fn reduce(state: State, num_ids: usize, max_time: usize, max_to_keep: usize) -> TotalPressure {
    //use the fn or see warnings
    utils::output_into_iter_io(
        io::stdout(),
        " ",
        &mut vec!["Ready,", "Steady, ", "Go"].iter(),
    );
    //calculate the maximum number of valves that it makes sense to open
    let max_valves_to_open: usize = state
        .valves
        .values()
        .filter(|valve| valve.flow_rate > 0)
        .count();

    let mut max_total_pressure_released = 0;

    let this_visits: &mut Vec<Visit> = &mut Vec::new();
    let next_visits: &mut Vec<Visit> = &mut Vec::new();
    this_visits.push(start_visit(&state, num_ids));

    let mut iteration = 0;
    while !this_visits.is_empty() {
        iteration += 1;

        'inner: for visit in this_visits.drain(..) {
            //Completions
            if let Some(total_pressure_released) = reached_max_time(&visit, max_time) {
                max_total_pressure_released =
                    cmp::max(max_total_pressure_released, total_pressure_released);
                continue 'inner;
            }
            if let Some(total_pressure_released) =
                all_valves_opened(max_valves_to_open, &visit, max_time)
            {
                max_total_pressure_released =
                    cmp::max(max_total_pressure_released, total_pressure_released);
                continue 'inner;
            }

            //Optional next paths
            for next_visit in turn_on_valve(&visit, &state) {
                next_visits.push(next_visit);
            }
            //Navigate to the others
            for next_visit in visit_neighbours(&visit, &state) {
                next_visits.push(next_visit);
            }
        }

        //limit the number taken forward
        if next_visits.len() > max_to_keep {
            println!(
                "Iteration {}: {} -> {}",
                iteration,
                next_visits.len(),
                max_to_keep
            );
            next_visits.sort_by(compare_fitness);
            next_visits.truncate(max_to_keep);
        }

        std::mem::swap(this_visits, next_visits);
    }

    max_total_pressure_released
}

fn start_visit(state: &State, num_ids: usize) -> Visit {
    let start_valve = state.valves.get(&String::from("AA")).unwrap();
    let mut ids: Vec<Rc<ValveID>> = Vec::new();
    for _ in 0..num_ids {
        ids.push(Rc::clone(&start_valve.id))
    }
    Visit::new(ids, 0, 0, 0, Rc::new(HashSet::new()))
}

///if this visit reaches the maximum time then return Some(total_pressure_released),
///otherwise, return None
fn reached_max_time(visit: &Visit, max_time: usize) -> Option<TotalPressure> {
    if visit.time_cost_to_reach >= max_time {
        return Some(visit.total_pressure_released);
    }
    None
}

///if this visit opens all valves then return Some(total_pressure_released) where
/// total_pressure_released will be calculated as applying the flow rate (which is now maximum)
/// until the time limit.
///If not all valves are open, return None
fn all_valves_opened(
    number_of_valves: usize,
    visit: &Visit,
    max_time: usize,
) -> Option<TotalPressure> {
    if visit.opened_valves.len() >= number_of_valves {
        let final_total_pressure_released = visit.total_pressure_released
            + visit.released_flow_rate * (max_time - visit.time_cost_to_reach);
        return Some(final_total_pressure_released);
    }
    None
}

///If we are able to turn on any valves, remain here and turn them on, returning
/// a Vector of next_visits,
///otherwise we can't turn it on and the returned vector will be empty
/// FIXME: yuck 1 and 2 handling
fn turn_on_valve(visit: &Visit, state: &State) -> Vec<Visit> {
    let mut next_visits: Vec<Visit> = Vec::new();
    if visit.ids.len() == 1 {
        let id = visit.ids.get(0).unwrap();
        let valve = state.valves.get(id).unwrap();
        if valve.flow_rate > 0 && !visit.opened_valves.contains(&valve.id) {
            let mut new_opened_valves = visit.opened_valves.deref().clone();
            new_opened_valves.insert(Rc::clone(id));

            next_visits.push(Visit::new(
                visit.ids.clone(),
                visit.time_cost_to_reach + 1,
                visit.released_flow_rate + valve.flow_rate,
                visit.total_pressure_released + visit.released_flow_rate,
                Rc::new(new_opened_valves),
            ));
        }
    }
    if visit.ids.len() == 2 {
        let id1 = visit.ids.get(0).unwrap();
        let valve1 = state.valves.get(id1).unwrap();
        let open1 = valve1.flow_rate > 0 && !visit.opened_valves.contains(&valve1.id);
        let id2 = visit.ids.get(1).unwrap();
        let valve2 = state.valves.get(id2).unwrap();
        let open2 = valve2.flow_rate > 0 && !visit.opened_valves.contains(&valve2.id);

        if open1 && open2 && (id1 != id2) {
            //open both
            let mut new_opened_valves = visit.opened_valves.deref().clone();
            new_opened_valves.insert(Rc::clone(id1));
            new_opened_valves.insert(Rc::clone(id2));

            next_visits.push(Visit::new(
                visit.ids.clone(), //staying where we are on both
                visit.time_cost_to_reach + 1,
                visit.released_flow_rate + valve1.flow_rate + valve2.flow_rate,
                visit.total_pressure_released + visit.released_flow_rate,
                Rc::new(new_opened_valves),
            ));
        } else if open1 || open2 {
            //open 1 navigate elsewhere for the other
            let (open_id, open_valve, move_valve) = if open1 {
                (id1, valve1, valve2)
            } else {
                (id2, valve2, valve1)
            };
            let mut new_opened_valves = visit.opened_valves.deref().clone();
            new_opened_valves.insert(Rc::clone(open_id));
            let new_opened_rc = Rc::new(new_opened_valves);
            for navigate_to_id in move_valve.tunnels_to.iter() {
                next_visits.push(Visit::new(
                    vec!(Rc::clone(open_id), Rc::clone(navigate_to_id)), //staying where we are on both
                    visit.time_cost_to_reach + 1,
                    visit.released_flow_rate + open_valve.flow_rate,
                    visit.total_pressure_released + visit.released_flow_rate,
                    Rc::clone(&new_opened_rc),
                ));
            }
        }
        //else no valves to open
    }
    next_visits
}

///Return a new visit for all the neighbours
fn visit_neighbours(visit: &Visit, state: &State) -> Vec<Visit> {
    let mut neighbours = Vec::new();
    if visit.ids.len() == 1 {
        let valve = state.valves.get(visit.ids.get(0).unwrap()).unwrap();
        for id in valve.tunnels_to.iter() {
            neighbours.push(Visit::new(
                vec![Rc::clone(id)],
                visit.time_cost_to_reach + 1,
                visit.released_flow_rate,
                visit.total_pressure_released + visit.released_flow_rate,
                Rc::clone(&visit.opened_valves),
            ));
        }
    }
    if visit.ids.len() == 2 {
        let valve1 = state.valves.get(visit.ids.get(0).unwrap()).unwrap();
        let valve2 = state.valves.get(visit.ids.get(1).unwrap()).unwrap();
        valve1
            .tunnels_to
            .iter()
            .flat_map(|id1| valve2.tunnels_to.iter().map(move |id2| (id1, id2)))
            .for_each(|(id1, id2)| {
                neighbours.push(Visit::new(
                    vec![Rc::clone(id1), Rc::clone(id2)],
                    visit.time_cost_to_reach + 1,
                    visit.released_flow_rate,
                    visit.total_pressure_released + visit.released_flow_rate,
                    Rc::clone(&visit.opened_valves),
                ));
            });
    }
    neighbours
}

///Prefer Higher total pressure, then higher released flow rate,
/// then more opened valves, then id
fn compare_fitness(visit1: &Visit, visit2: &Visit) -> Ordering {
    Ordering::Equal
        .then_with(|| {
            visit2
                .total_pressure_released
                .cmp(&visit1.total_pressure_released)
        })
        .then_with(|| visit2.released_flow_rate.cmp(&visit1.released_flow_rate))
        .then_with(|| visit2.opened_valves.len().cmp(&visit1.opened_valves.len()))
}
