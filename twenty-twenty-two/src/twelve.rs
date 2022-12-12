use std::{io::Error, collections::HashMap};

use crate::utils;

const FILE_NAME: &str = "12/input.txt";

type PathLength = usize;

pub fn _12a() -> Result<PathLength, Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        Map::new(),
        accumulate,
        find_shortest_path1
    )
}

pub fn _12b() -> Result<PathLength, Error> {
    utils::process_file(
        FILE_NAME,
        parse_line,
        Map::new(),
        accumulate,
        find_shortest_path2,
    )
}

/// Parse a line, trying to find the start and end pos in the line
fn parse_line(line: String) -> (Vec<char>, Option<usize>, Option<usize>)  {
    let mut row = Vec::with_capacity(line.len());
    let mut start_pos = None;
    let mut end_pos = None;
    for (index, c) in line.chars().enumerate() {
        match c {
            'S' => {
                start_pos = Some(index);
                row.push('a');
            },
            'E' => {
                end_pos = Some(index);
                row.push('z');
            }
            _ => row.push(c),
        }
    }
    (row, start_pos, end_pos)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<char>>,
    start_pos: Option<Pos>,
    end_pos: Option<Pos>,
}

impl Map {
    pub fn new() -> Map {
        Map{
            rows: Vec::new(),
            start_pos: None,
            end_pos: None,
        }
    }
}

fn accumulate(mut map: Map, row_details: (Vec<char>, Option<usize>, Option<usize>)) -> Map {
    let (row, start_pos, end_pos) = row_details;
    map.rows.push(row);
    let y = map.rows.len() - 1;
    for pos in start_pos.iter() {
        map.start_pos = Some(Pos::new(*pos, y));
    }
    for pos in end_pos.iter() {
        map.end_pos = Some(Pos::new(*pos, y));
    }
    map
}

struct PosToVisit {
    pos: Pos,
    distance: PathLength,
}


impl PosToVisit {
    pub fn new(pos: Pos, distance: PathLength) -> PosToVisit {
        PosToVisit { pos, distance }
    }
}

fn find_shortest_path1(map: Map) -> PathLength {
    let visited_distances = shortest_path(
        &map, map.start_pos.unwrap(), 1,
        |_, _, _| {}
    );

    // //perform a BFS
    // let mut visited_distances: HashMap<Pos, PathLength> = HashMap::new();
    // let mut to_visit: Vec<PosToVisit> = Vec::new();
    // to_visit.push(PosToVisit { pos: map.start_pos.unwrap(), distance: 0 });

    // while !to_visit.is_empty() {
    //     let next_to_visit = to_visit.pop().unwrap();
    //     //did we already visit in the same or fewer?
    //     let already_visited_length = visited_distances.get(&next_to_visit.pos);
    //     if already_visited_length.is_some() && *already_visited_length.unwrap() <= next_to_visit.distance {
    //         continue;
    //     }
    //     visited_distances.insert(next_to_visit.pos, next_to_visit.distance);
    //     //now work out where to visit next
    //     visit_north(&map, &next_to_visit.pos, next_to_visit.distance + 1, &visited_distances, &mut to_visit);
    //     visit_south(&map, &next_to_visit.pos, next_to_visit.distance + 1, &visited_distances, &mut to_visit);
    //     visit_east(&map, &next_to_visit.pos, next_to_visit.distance + 1, &visited_distances, &mut to_visit);
    //     visit_west(&map, &next_to_visit.pos, next_to_visit.distance + 1, &visited_distances, &mut to_visit);
    // }

    let end_pos = map.end_pos.unwrap();
    *visited_distances.get(&end_pos).unwrap()
}

fn find_shortest_path2(map: Map) -> PathLength {
    //perform a BFS but we are doing it in the other direction and finding the shortest to an 'a'
    let mut shortest_distance = PathLength::MAX;
    let _visited_distances = shortest_path(
        &map, map.end_pos.unwrap(), -1,
        |map, pos, distance| {
            let height = map.rows.get(pos.y).unwrap().get(pos.x).unwrap();
            if *height == 'a' && *distance < shortest_distance {
                shortest_distance = *distance;
            }
        }
    );

    shortest_distance
}

/// Find the shortest path from the start_pos in the visit direction (1 for upwards, -1 for downwards).
///
/// This will find a path to all navigable positions.
///
/// The returned map can be used to get the shortest path to a particular position.
///
/// To be notified whenever a position is visited, use the on_visit callback.
fn shortest_path<F>(map: &Map, start_pos: Pos, visit_direction: i32,
                    mut on_visit: F) -> HashMap<Pos, PathLength>
    where F: FnMut(&Map, &Pos, &PathLength) -> ()
{
    let mut visited_distances: HashMap<Pos, PathLength> = HashMap::new();
    let mut to_visit: Vec<PosToVisit> = Vec::new();
    to_visit.push(PosToVisit { pos: start_pos, distance: 0 });

    while !to_visit.is_empty() {
        let next_to_visit = to_visit.pop().unwrap();
        //did we already visit in the same or fewer?
        let already_visited_length = visited_distances.get(&next_to_visit.pos);
        if already_visited_length.is_some() && *already_visited_length.unwrap() <= next_to_visit.distance {
            continue;
        }
        on_visit(map, &next_to_visit.pos, &next_to_visit.distance);
        visited_distances.insert(next_to_visit.pos, next_to_visit.distance);
        //now work out where to visit next
        visit_north(map, visit_direction, &next_to_visit.pos, next_to_visit.distance + 1, &visited_distances, &mut to_visit);
        visit_south(map, visit_direction, &next_to_visit.pos, next_to_visit.distance + 1, &visited_distances, &mut to_visit);
        visit_east(map, visit_direction, &next_to_visit.pos, next_to_visit.distance + 1, &visited_distances, &mut to_visit);
        visit_west(map, visit_direction, &next_to_visit.pos, next_to_visit.distance + 1, &visited_distances, &mut to_visit);
    }

    visited_distances
}

fn visit_north(map: &Map, visit_direction: i32, pos: &Pos, distance: PathLength, visited_distances: &HashMap<Pos, PathLength>, to_visit: &mut Vec<PosToVisit>) {
    if pos.y <= 0 {
        return;
    }
    let new_pos = Pos::new(pos.x, pos.y - 1);
    visit_pos(map, visit_direction, pos, new_pos, distance, visited_distances, to_visit);
}

fn visit_south(map: &Map, visit_direction: i32, pos: &Pos, distance: PathLength, visited_distances: &HashMap<Pos, PathLength>, to_visit: &mut Vec<PosToVisit>) {
    if !(pos.y < map.rows.len() - 1) {
        return;
    }
    let new_pos = Pos::new(pos.x, pos.y + 1);
    visit_pos(map, visit_direction, pos, new_pos, distance, visited_distances, to_visit)
}

fn visit_east(map: &Map, visit_direction: i32, pos: &Pos, distance: PathLength, visited_distances: &HashMap<Pos, PathLength>, to_visit: &mut Vec<PosToVisit>) {
    if !(pos.x < map.rows.first().unwrap().len() - 1) {
        return;
    }
    let new_pos = Pos::new(pos.x + 1, pos.y);
    visit_pos(map, visit_direction, pos, new_pos, distance, visited_distances, to_visit)

}

fn visit_west(map: &Map, visit_direction: i32, pos: &Pos, distance: PathLength, visited_distances: &HashMap<Pos, PathLength>, to_visit: &mut Vec<PosToVisit>) {
    if pos.x <= 0 {
        return;
    }
    let new_pos = Pos::new(pos.x - 1, pos.y);
    visit_pos(map, visit_direction, pos, new_pos, distance, visited_distances, to_visit)
}

fn visit_pos(map: &Map, visit_direction: i32, pos: &Pos, new_pos: Pos, distance_to_new: PathLength, visited_distances: &HashMap<Pos, PathLength>, to_visit: &mut Vec<PosToVisit>) {
    //did we visit already in fewer??
    let visited_distance = visited_distances.get(&new_pos);
    if visited_distance.is_some() && *visited_distance.unwrap() <= distance_to_new {
        return;
    }
    //can we visit (height is not more than 1 more)
    let pos_height = map.rows.get(pos.y).unwrap().get(pos.x).unwrap();
    let new_pos_height = map.rows.get(new_pos.y).unwrap().get(new_pos.x).unwrap();
    let height_diff = (*new_pos_height as i32 - *pos_height as i32) * visit_direction;
    if height_diff <= 1 {
        to_visit.push(PosToVisit::new(new_pos, distance_to_new));
    }
}
