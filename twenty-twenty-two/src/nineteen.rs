use std::cmp::Ordering;
use std::fmt::Display;

use crate::utils;

const FILE_NAME: &str = "19/input.txt";
//const FILE_NAME: &str = "19/test_input.txt";

pub fn _19a() -> Result<usize, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce1)
}

pub fn _19b() -> Result<usize, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce2)
}

const BLUEPRINT: &str = "Blueprint ";
const ORE_ROBOT: &str = " Each ore robot costs ";
const CLAY_ROBOT: &str = "ore. Each clay robot costs ";
const OBSIDIAN_ROBOT_ORE: &str = "ore. Each obsidian robot costs ";
const OBSIDIAN_ROBOT_CLAY: &str = "ore and ";
const GEODE_ROBOT_ORE: &str = "clay. Each geode robot costs ";
const GEODE_ROBOT_OBSIDIAN: &str = "ore and ";

struct Costs {
    blueprint_id: usize,
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

impl Costs {
    pub fn new(
        blueprint_id: usize,
        ore_robot_ore_cost: usize,
        clay_robot_ore_cost: usize,
        obsidian_robot_ore_cost: usize,
        obsidian_robot_clay_cost: usize,
        geode_robot_ore_cost: usize,
        geode_robot_obsidian_cost: usize,
    ) -> Costs {
        Costs {
            blueprint_id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        }
    }
}

impl Display for Costs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}:{}{} {}{} {}{} {}{} {}{} {}{} obsidian.",
            BLUEPRINT,
            self.blueprint_id,
            ORE_ROBOT,
            self.ore_robot_ore_cost,
            CLAY_ROBOT,
            self.clay_robot_ore_cost,
            OBSIDIAN_ROBOT_ORE,
            self.obsidian_robot_ore_cost,
            OBSIDIAN_ROBOT_CLAY,
            self.obsidian_robot_clay_cost,
            GEODE_ROBOT_ORE,
            self.geode_robot_ore_cost,
            GEODE_ROBOT_OBSIDIAN,
            self.geode_robot_obsidian_cost
        )
    }
}

fn parse_line(line: String) -> Costs {
    //Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 3 ore and 14 obsidian.
    let mut chars = line.chars();
    utils::skip(&mut chars, BLUEPRINT.len());
    let blueprint_id: usize = utils::parse_next_number(&mut chars).unwrap();
    utils::skip(&mut chars, ORE_ROBOT.len());
    let ore_robot_ore_cost: usize = utils::parse_next_number(&mut chars).unwrap();
    utils::skip(&mut chars, CLAY_ROBOT.len());
    let clay_robot_ore_cost: usize = utils::parse_next_number(&mut chars).unwrap();
    utils::skip(&mut chars, OBSIDIAN_ROBOT_ORE.len());
    let obsidian_robot_ore_cost: usize = utils::parse_next_number(&mut chars).unwrap();
    utils::skip(&mut chars, OBSIDIAN_ROBOT_CLAY.len());
    let obsidian_robot_clay_cost: usize = utils::parse_next_number(&mut chars).unwrap();
    utils::skip(&mut chars, GEODE_ROBOT_ORE.len());
    let geode_robot_ore_cost: usize = utils::parse_next_number(&mut chars).unwrap();
    utils::skip(&mut chars, GEODE_ROBOT_OBSIDIAN.len());
    let geode_robot_obsidian_cost: usize = utils::parse_next_number(&mut chars).unwrap();
    Costs::new(
        blueprint_id,
        ore_robot_ore_cost,
        clay_robot_ore_cost,
        obsidian_robot_ore_cost,
        obsidian_robot_clay_cost,
        geode_robot_ore_cost,
        geode_robot_obsidian_cost,
    )
}

struct State {
    costs_to_check: Vec<Costs>,
}

impl State {
    pub fn new() -> State {
        State {
            costs_to_check: Vec::new(),
        }
    }
}

fn accumulate(mut state: State, costs: Costs) -> State {
    println!("{}", costs);
    state.costs_to_check.push(costs);
    state
}

fn reduce1(state: State) -> usize {
    let mut total_quality_level = 0;
    for costs in state.costs_to_check {
        let quality = run_simulation(&costs, 24);
        println!("Blueprint {}: Quality: {}", costs.blueprint_id, quality);
        total_quality_level += costs.blueprint_id * quality;
    }
    total_quality_level
}

fn reduce2(state: State) -> usize {
    let mut total = 1;
    for i in 0..3 {
        let costs = state.costs_to_check.get(i).unwrap();
        let quality = run_simulation(costs, 32);
        println!("Blueprint {}: Quality: {}", costs.blueprint_id, quality);
        total *= quality;
    }
    total
}

struct Step {
    minute: usize,

    ore_collecting_robots: usize,
    clay_collecting_robots: usize,
    obsidian_collecting_robots: usize,
    geode_cracking_robots: usize,

    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl Step {
    pub fn new() -> Step {
        Step {
            minute: 1,

            ore_collecting_robots: 1,
            clay_collecting_robots: 0,
            obsidian_collecting_robots: 0,
            geode_cracking_robots: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
}

const MAX_TO_KEEP: usize = 200;

fn run_simulation(costs: &Costs, max_steps: usize) -> usize {

    let mut max_opened_geodes: usize = 0;

    let this_run_steps: &mut Vec<Step> = &mut Vec::new();
    this_run_steps.push(Step::new());
    let next_run_steps: &mut Vec<Step> = &mut Vec::new();

    while !this_run_steps.is_empty() {
        let mut minute = 0;
        'inner: for step in this_run_steps.iter() {
            minute = step.minute;
            if step.minute >= max_steps {
                //make final resource collections
                let final_step = build_nothing(&step);
                max_opened_geodes = std::cmp::max(max_opened_geodes, final_step.geodes);
                continue 'inner;
            }

            if let Some(new_step) = build_ore_collecting_robot(costs, &step) {
                next_run_steps.push(new_step);
            }
            if let Some(new_step) = build_clay_collecting_robot(costs, &step) {
                next_run_steps.push(new_step);
            }
            if let Some(new_step) = build_obsidian_collecting_robot(costs, &step) {
                next_run_steps.push(new_step);
            }
            if let Some(new_step) = build_geode_cracking_robot(costs, &step) {
                next_run_steps.push(new_step);
            }
            //wait for more collection
            let new_step = build_nothing(&step);
            next_run_steps.push(new_step);
        }

        this_run_steps.clear();
        //limit number of next run steps
        if next_run_steps.len() > MAX_TO_KEEP {
            print!("Min {}: {} -> {},", minute, next_run_steps.len(), MAX_TO_KEEP);
            next_run_steps.sort_by(sort_steps);
            next_run_steps.truncate(MAX_TO_KEEP);
        }
        //swap pointers
        std::mem::swap(this_run_steps, next_run_steps);
    }
    println!("");

    max_opened_geodes
}

fn sort_steps(s1: &Step, s2: &Step) -> Ordering {
    let geode_robot_ordering = compare(s2.geode_cracking_robots, s1.geode_cracking_robots);
    if geode_robot_ordering != Ordering::Equal {
        return geode_robot_ordering;
    }
    let geodes_ordering = compare(s2.geodes, s1.geodes);
    if geodes_ordering != Ordering::Equal {
        return geodes_ordering;
    }
    let obsidian_robots = compare(s2.obsidian_collecting_robots, s1.obsidian_collecting_robots);
    if obsidian_robots != Ordering::Equal {
        return obsidian_robots;
    }
    let obsidian_ordering = compare(s2.obsidian, s1.obsidian);
    if obsidian_ordering != Ordering::Equal {
        return obsidian_ordering;
    }
    let clay_robots = compare(s2.clay_collecting_robots, s1.clay_collecting_robots);
    if clay_robots != Ordering::Equal {
        return clay_robots;
    }
    let clay_ordering = compare(s2.clay, s1.clay);
    if clay_ordering != Ordering::Equal {
        return clay_ordering;
    }
    let ore_robots = compare(s2.ore_collecting_robots, s1.ore_collecting_robots);
    if ore_robots != Ordering::Equal {
        return ore_robots;
    }
    compare(s2.ore, s1.ore)
}


fn compare(s1: usize, s2: usize) -> Ordering {
    if s1 < s2 {
        Ordering::Less
    } else if s1 > s2  {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn build_ore_collecting_robot(costs: &Costs, step: &Step) -> Option<Step> {
    if costs.ore_robot_ore_cost <= step.ore {
        Some(Step {
            minute: step.minute + 1,

            ore: step.ore - costs.ore_robot_ore_cost + step.ore_collecting_robots,
            clay: step.clay + step.clay_collecting_robots,
            obsidian: step.obsidian + step.obsidian_collecting_robots,
            geodes: step.geodes + step.geode_cracking_robots,

            ore_collecting_robots: step.ore_collecting_robots + 1,

            ..*step
        })
    } else {
        None
    }
}

fn build_clay_collecting_robot(costs: &Costs, step: &Step) -> Option<Step> {
    if costs.clay_robot_ore_cost <= step.ore {
        Some(Step {
            minute: step.minute + 1,

            ore: step.ore - costs.clay_robot_ore_cost + step.ore_collecting_robots,
            clay: step.clay + step.clay_collecting_robots,
            obsidian: step.obsidian + step.obsidian_collecting_robots,
            geodes: step.geodes + step.geode_cracking_robots,

            clay_collecting_robots: step.clay_collecting_robots + 1,

            ..*step
        })
    } else {
        None
    }
}

fn build_obsidian_collecting_robot(costs: &Costs, step: &Step) -> Option<Step> {
    if costs.obsidian_robot_ore_cost <= step.ore && costs.obsidian_robot_clay_cost <= step.clay {
        Some(Step {
            minute: step.minute + 1,

            ore: step.ore - costs.obsidian_robot_ore_cost + step.ore_collecting_robots,
            clay: step.clay - costs.obsidian_robot_clay_cost + step.clay_collecting_robots,
            obsidian: step.obsidian + step.obsidian_collecting_robots,
            geodes: step.geodes + step.geode_cracking_robots,

            obsidian_collecting_robots: step.obsidian_collecting_robots + 1,

            ..*step
        })
    } else {
        None
    }
}

fn build_geode_cracking_robot(costs: &Costs, step: &Step) -> Option<Step> {
    if costs.geode_robot_ore_cost <= step.ore && costs.geode_robot_obsidian_cost <= step.obsidian {
        Some(Step {
            minute: step.minute + 1,

            ore: step.ore - costs.geode_robot_ore_cost + step.ore_collecting_robots,
            clay: step.clay + step.clay_collecting_robots,
            obsidian: step.obsidian - costs.geode_robot_obsidian_cost
                + step.obsidian_collecting_robots,
            geodes: step.geodes + step.geode_cracking_robots,

            geode_cracking_robots: step.geode_cracking_robots + 1,

            ..*step
        })
    } else {
        None
    }
}

fn build_nothing(step: &Step) -> Step {
    Step {
        minute: step.minute + 1,

        ore: step.ore + step.ore_collecting_robots,
        clay: step.clay + step.clay_collecting_robots,
        obsidian: step.obsidian + step.obsidian_collecting_robots,
        geodes: step.geodes + step.geode_cracking_robots,

        ..*step
    }
}
