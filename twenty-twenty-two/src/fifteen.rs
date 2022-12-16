use core::panic;
use std::str::Chars;
use std::cmp::{min, max};

use crate::utils;

const FILE_NAME: &str = "15/input.txt";
// const FILE_NAME: &str = "15/test_input.txt";
// const FILE_NAME: &str = "15/tiny_input.txt";

pub fn _15a() -> Result<i64, std::io::Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new(),
        accumulate,
        reduce1
    )
}

pub fn _15b() -> Result<i64, std::io::Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        State::new(),
        accumulate,
        reduce2
    )
}

type Scale = i64;

struct SensorAndBeacon {
    sensor_x: Scale,
    sensor_y: Scale,
    beacon_x: Scale,
    beacon_y: Scale,
    radius: Scale,
    min_coverage_x: Scale,
    min_coverage_y: Scale,
    max_coverage_x: Scale,
    max_coverage_y: Scale,
}

impl SensorAndBeacon {
    pub fn new(sensor_x: Scale, sensor_y: Scale,
        beacon_x: Scale, beacon_y: Scale) -> SensorAndBeacon {
            let radius = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            let min_coverage_x = sensor_x - radius;
            let min_coverage_y  = sensor_y - radius;
            let max_coverage_x = sensor_x + radius;
            let max_coverage_y = sensor_y + radius;
            SensorAndBeacon {
                sensor_x, sensor_y,
                beacon_x, beacon_y,
                radius,
                min_coverage_x, min_coverage_y,
                max_coverage_x, max_coverage_y
            }
    }

    ///returns an optional Scale value that signifies the following:
    /// None: if (x,y) is not covered
    /// Some(x2, beacon): if (x,y) is covered:
    ///   where x2 is the max x that is covered at row y
    ///   if beacon = true, then the beacon is in one of the squares
    pub fn covers(self: &Self, x: Scale, y: Scale) -> Option<(Scale, bool)> {
        if x < self.min_coverage_x || x > self.max_coverage_x {
            return None;
        }

        let x_dist = (self.sensor_x - x).abs();
        let y_dist = (self.sensor_y - y).abs();
        let distance = x_dist + y_dist;
        if distance > self.radius {
            return None;
        }
        let last_x = self.sensor_x + x_dist;
        let includes_beacon = self.beacon_y == y && self.beacon_x >= x;
        Some((last_x, includes_beacon))
    }

    pub fn coverage(self: &Self, row: Scale) -> Option((Scale, Scale)) {
        let row_dist = (row - self.sensor_y).abs();
        let x_dist = self.radius - row_dist;
        if (x_dist < 0) {
            return None;
        }
        return Some((self.sensor_x - x_dist, self.sensor_x + x_dist))
    }

}

fn skip(chars: &mut Chars<'_>, n: usize) {
    for _ in 0..n {
        chars.next();
    }
}

fn parse_next_number(chars: &mut Chars<'_>) -> Scale {
    let mut s = String::new();
    for c in chars {
        let finish = match c {
            ',' | ':' => true,
            _ => {
                s.push(c);
                false
            },
        };
        if finish {
            break;
        }
    }
    s.parse().unwrap()
}

fn parse_line(line: String) -> SensorAndBeacon {
    //Example line:
    //Sensor at x=2389280, y=2368338: closest beacon is at x=2127703, y=2732666
    let mut chars = line.chars();
    skip(&mut chars, 12); //"Sensor at x="
    let sensor_x = parse_next_number(&mut chars); //will parse up to ","
    skip(&mut chars, 3); //" y="
    let sensor_y = parse_next_number(&mut chars); //will parse up to ":"
    skip(&mut chars, 24); //" closest beacon is at x="
    let beacon_x = parse_next_number(&mut chars); //will parse up to ","
    skip(&mut chars, 3);//" y="
    let beacon_y = parse_next_number(&mut chars);//eof
    SensorAndBeacon::new(sensor_x, sensor_y, beacon_x, beacon_y)
}

#[derive(Debug)]
struct Bounds {
    min_x: Scale,
    min_y: Scale,
    max_x: Scale,
    max_y: Scale,
}

impl Bounds {
    pub fn new() -> Bounds {
        Bounds {
            min_x: Scale::MAX,
            min_y: Scale::MAX,
            max_x: Scale::MIN,
            max_y: Scale::MIN,
        }
    }
}

struct State {
    sensors: Vec<SensorAndBeacon>,
    bounds: Bounds,
}

impl State {
    pub fn new() -> State {
        State {
            sensors: Vec::new(),
            bounds: Bounds::new(),
        }
    }
}

fn accumulate(mut state: State, sensor_and_beacon: SensorAndBeacon) -> State {
    state.bounds.min_x = min(state.bounds.min_x, sensor_and_beacon.min_coverage_x);
    state.bounds.min_y = min(state.bounds.min_y, sensor_and_beacon.min_coverage_y);
    state.bounds.max_x = max(state.bounds.max_x, sensor_and_beacon.max_coverage_x);
    state.bounds.max_y = max(state.bounds.max_y, sensor_and_beacon.max_coverage_y);
    state.sensors.push(sensor_and_beacon);
    state
}

// fn output_state(state: &State) {
//     println!("");
//     println!("{:?}", state.bounds);
//     for y in state.bounds.min_y..(state.bounds.max_y + 1) {
//         for x in state.bounds.min_x..(state.bounds.max_x + 1) {
//             match state.sensors.get(0).unwrap().covers(x, y) {
//                 Some(_) => print!("#"),
//                 None => print!("."),
//             }
//         }
//         println!("");
//     }
//     println!("");
// }

fn reduce1(state: State) -> i64 {
    // output_state(&state);
    count_non_covered_positions(&state, 2_000_000)
    // count_non_covered_positions(&state, 10)
    // count_non_covered_positions(&state, 2)
}

fn count_non_covered_positions(state: &State, row: Scale) -> i64 {
    let mut count = 0;
    let mut x = state.bounds.min_x;
    'outer: while x <= state.bounds.max_x {
        for sensor in state.sensors.iter() {
            if let Some((max_x_covered, includes_beacon)) = sensor.covers(x, row) {
                count += max_x_covered - x;
                if !includes_beacon {
                    count += 1
                }
                x = max_x_covered + 1;
                continue 'outer;
            }
        }
        //If we get here it could contain a beacon
        x += 1;
    }
    count
}

const MAX_X_Y: i64 = 4_000_000;

fn reduce2(state: State) -> i64 {
    for y in 0..(MAX_X_Y + 1) {
        if y % 1000 == 0 {
            println!("{}", y);
        }
        let mut x = 0;
        'x_loop: while x <= (MAX_X_Y + 1) {
            for sensor in state.sensors.iter() {
                if let Some((max_x_covered, _)) = sensor.covers(x, y) {
                    x = max_x_covered + 1;
                    continue 'x_loop;
                }
            }
            println!("({},{})", x, y);
            return x * 4_000_000 + y;
            // x += 1;
        }
    }
    panic!("Not found it")
}
