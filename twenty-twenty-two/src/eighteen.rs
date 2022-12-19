use std::fmt::Display;
use std::collections::{BTreeMap, HashSet, VecDeque};

use crate::utils;

const FILE_NAME: &str = "18/input.txt";
// const FILE_NAME: &str = "18/test_input.txt";
// const FILE_NAME: &str = "18/my_testing_input.txt";

pub fn _18a() -> Result<u32, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce1)
}

pub fn _18b() -> Result<u32, std::io::Error> {
    utils::process_file(FILE_NAME, parse_line, State::new(), accumulate, reduce2)
}

type CoordScale = i32;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord3 {
    x: CoordScale,
    y: CoordScale,
    z: CoordScale,
}

impl Coord3 {
    pub fn new(x: CoordScale, y: CoordScale, z: CoordScale) -> Coord3 {
        Coord3 { x, y, z }
    }

    pub fn neighbours(&self) -> [Coord3; 6] {
        [
            Coord3::new(self.x + 1, self.y, self.z),
            Coord3::new(self.x - 1, self.y, self.z),
            Coord3::new(self.x, self.y + 1, self.z),
            Coord3::new(self.x, self.y - 1, self.z),
            Coord3::new(self.x, self.y, self.z + 1),
            Coord3::new(self.x, self.y, self.z - 1),
        ]
    }
}

impl Display for Coord3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

fn parse_line(line: String) -> Coord3 {
    let mut chars = line.chars();
    let x: CoordScale = utils::parse_next_number(&mut chars).unwrap();
    let y: CoordScale = utils::parse_next_number(&mut chars).unwrap();
    let z: CoordScale = utils::parse_next_number(&mut chars).unwrap();
    Coord3::new(x, y, z)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord2 {
    _1: CoordScale,
    _2: CoordScale,
}

impl Coord2 {
    pub fn new(_1: CoordScale, _2: CoordScale) -> Coord2 {
        Coord2 { _1, _2 }
    }
}

impl Display for Coord2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self._1, self._2)
    }
}

struct State {
    all_cube_coords: HashSet<Coord3>,
    xy_coords_by_z: BTreeMap<CoordScale, HashSet<Coord2>>,
    xz_coords_by_y: BTreeMap<CoordScale, HashSet<Coord2>>,
    yz_coords_by_x: BTreeMap<CoordScale, HashSet<Coord2>>,
}

impl State {
    pub fn new() -> State {
        State {
            all_cube_coords: HashSet::new(),
            xy_coords_by_z: BTreeMap::new(),
            xz_coords_by_y: BTreeMap::new(),
            yz_coords_by_x: BTreeMap::new(),
        }
    }
}

fn accumulate(mut state: State, coord: Coord3) -> State {
    //println!("{}", coord);
    let coords_z = state.xy_coords_by_z.entry(coord.z).or_insert(HashSet::new());
    coords_z.insert(Coord2::new(coord.x,coord.y));
    let coords_y = state.xz_coords_by_y.entry(coord.y).or_insert(HashSet::new());
    coords_y.insert(Coord2::new(coord.x, coord.z));
    let coords_x = state.yz_coords_by_x.entry(coord.x).or_insert(HashSet::new());
    coords_x.insert(Coord2::new(coord.y, coord.z));
    state.all_cube_coords.insert(coord);
    state
}

fn xy_by_z_coord3_builder(coord: &Coord2, slice: CoordScale) -> Coord3 {
    Coord3::new(coord._1, coord._2, slice)
}

fn xz_by_y_coord3_builder(coord: &Coord2, slice: CoordScale) -> Coord3 {
    Coord3::new(coord._1, slice, coord._2)
}

fn yz_by_x_coord3_builder(coord: &Coord2, slice: CoordScale) -> Coord3 {
    Coord3::new(slice, coord._1, coord._2)
}

fn reduce(state: &State) -> u32 {
    println!("There are {} coords", state.all_cube_coords.len());

    let mut num_faces = count_faces(
        &state.all_cube_coords,
        &state.xy_coords_by_z,
        false,
        xy_by_z_coord3_builder,
    );
    num_faces += count_faces(
        &state.all_cube_coords,
        &state.xy_coords_by_z,
        true,
        xy_by_z_coord3_builder,
    );

    num_faces += count_faces(
        &state.all_cube_coords,
        &state.xz_coords_by_y,
        false,
        xz_by_y_coord3_builder,
    );
    num_faces += count_faces(
        &state.all_cube_coords,
        &state.xz_coords_by_y,
        true,
        xz_by_y_coord3_builder,
    );

    num_faces += count_faces(
        &state.all_cube_coords,
        &state.yz_coords_by_x,
        false,
        yz_by_x_coord3_builder,
    );
    num_faces += count_faces(
        &state.all_cube_coords,
        &state.yz_coords_by_x,
        true,
        yz_by_x_coord3_builder,
    );

    num_faces
}

///Count the faces.
/// coord3_builder will builds a [Coord3] from the current Coord2 and the previous 'slice'
fn count_faces(
    all_cube_coords: &HashSet<Coord3>,
    by_axis_map: &BTreeMap<CoordScale, HashSet<Coord2>>,
    reverse: bool,
    coord3_builder: fn(&Coord2, CoordScale) -> Coord3,
) -> u32 {
    fn count_slice_faces(
        all_cube_coords: &HashSet<Coord3>,
        coords: &HashSet<Coord2>,
        previous_slice: CoordScale,
        coord3_builder: fn(&Coord2, CoordScale) -> Coord3,
    ) -> u32 {
        let mut num_faces = 0;
        for coord in coords.iter() {
            //does this have a previous cube?
            let prev_coord3 = coord3_builder(coord, previous_slice);
            if !all_cube_coords.contains(&prev_coord3) {
                num_faces += 1;
            }
        }
        num_faces
    }

    let mut total_faces = 0;
    if reverse {
        for (slice, coords) in by_axis_map.iter().rev() {
            total_faces += count_slice_faces(
                all_cube_coords,
                coords,
                *slice + 1,
                coord3_builder,
            );
        }
    } else {
        for (slice, coords) in by_axis_map.iter() {
            total_faces += count_slice_faces(
                all_cube_coords,
                coords,
                *slice - 1,
                coord3_builder,
            );
        }
    }
    total_faces
}

fn reduce1(state: State) -> u32 {
    reduce(&state)
}

struct Bounds {
    min_x: CoordScale,
    max_x: CoordScale,
    min_y: CoordScale,
    max_y: CoordScale,
    min_z: CoordScale,
    max_z: CoordScale,
}

impl Bounds {
    pub fn new(
        min_x: CoordScale, max_x: CoordScale,
        min_y: CoordScale, max_y: CoordScale,
        min_z: CoordScale, max_z: CoordScale) -> Bounds {
            Bounds {
                min_x, max_x,
                min_y, max_y,
                min_z, max_z
            }
    }
}

fn reduce2(state: State) -> u32 {
    let all_faces = reduce(&state);

    let bounds = Bounds::new(
        *state.yz_coords_by_x.first_key_value().unwrap().0,
        *state.yz_coords_by_x.last_key_value().unwrap().0,
        *state.xz_coords_by_y.first_key_value().unwrap().0,
        *state.xz_coords_by_y.last_key_value().unwrap().0,
        *state.xy_coords_by_z.first_key_value().unwrap().0,
        *state.xy_coords_by_z.last_key_value().unwrap().0,
    );

    let mut visited_coords: HashSet<Coord3> = HashSet::new();
    let mut all_bubble_coords: HashSet<Coord3> = HashSet::new();
    for x in state.yz_coords_by_x.keys() {
        for y in state.xz_coords_by_y.keys() {
            for z in state.xy_coords_by_z.keys() {
                let cube_coord = Coord3::new(*x, *y, *z);
                look_for_an_escape(&state, &bounds, &mut visited_coords, &mut all_bubble_coords, cube_coord);
            }
        }
    }

    //run a reduce on the bubble state
    let mut bubble_state = State::new();
    for coord in all_bubble_coords {
        bubble_state = accumulate(bubble_state, coord);
    }

    for coord in state.all_cube_coords.intersection(&bubble_state.all_cube_coords) {
        println!("!!Intesection: {}!!", coord);
    }

    let bubble_faces = reduce(&bubble_state);

    all_faces - bubble_faces
}

///Look for a way to escape the shape - if it is not possible to escape, then add to the bubble coords (and anything else in this bubble)
fn look_for_an_escape(
    state: &State,
    bounds: &Bounds,
    visited_coords: &mut HashSet<Coord3>,
    bubble_coords: &mut HashSet<Coord3>,
    coord: Coord3
) {
    //Quick exit if we've already been here
    if visited_coords.contains(&coord) {
        return;
    }
    //Quick exit if this is a populated cube - we're only interested in spaces
    if state.all_cube_coords.contains(&coord) {
        return;
    }
    //not populated, look for an escape route
    let mut stack: VecDeque<Coord3> = VecDeque::new();
    let mut bubble_candidates: HashSet<Coord3> = HashSet::new();
    let mut escaped = false;
    visited_coords.insert(coord);
    stack.push_front(coord);
    while !stack.is_empty() {
        let coord = stack.pop_front().unwrap();

        if out_of_bounds(&bounds, &coord) {
            //escaped!
            escaped = true;
            continue;
        }

        if !escaped {
            //not escaped yet, it's a bubble candidate
            bubble_candidates.insert(coord);
        }

        //visit the empty neighbours, we've not visited yet
        coord.neighbours().iter()
            .filter(|co| !state.all_cube_coords.contains(*co))
            .for_each(|co| {
                if !visited_coords.contains(co) {
                    visited_coords.insert(*co);
                    stack.push_front(*co);
                }
            });
    }

    if !escaped {
        for coord in bubble_candidates.drain() {
            bubble_coords.insert(coord);
        }
    }
}

fn out_of_bounds(bounds: &Bounds, coord: &Coord3) -> bool {
    coord.x > bounds.max_x || coord.x < bounds.min_x ||
    coord.y > bounds.max_y || coord.y < bounds.min_y ||
    coord.z > bounds.max_z || coord.z < bounds.min_z
}