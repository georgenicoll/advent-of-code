use std::io::Error;
use std::convert::identity;
use std::cmp;
use crate::utils;


const FILE_NAME: &str = "8/input.txt";

pub fn _8a() -> Result<u64, Error>{
    utils::process_file(
        FILE_NAME,
        identity,
        Trees::new(),
        accumulator,
        reduce1
    )
}

pub fn _8b() -> Result<u64, Error> {
    utils::process_file(
        FILE_NAME,
        identity,
        Trees::new(),
        accumulator,
        reduce2
    )
}

struct Tree {
    height: i8,
    highest_east: i8,
    highest_west: i8,
    highest_north: i8,
    highest_south: i8,
}

impl Tree {
    pub fn new(height: i8, highest_east: i8, highest_north: i8) -> Tree {
        Tree {
            height: height,
            highest_east,
            highest_west: -1,
            highest_north,
            highest_south: -1,
        }
    }
}

struct Trees {
    trees: Vec<Vec<Tree>>
}

impl Trees {
    pub fn new() -> Trees {
        Trees {
            trees: Vec::new(),
        }
    }
}

fn accumulator(mut trees: Trees, line: String) -> Trees {
    let row_north = trees.trees.last();
    let mut highest_east: i8 = -1;
    let mut tree_row: Vec<Tree> = Vec::with_capacity(line.len());
    for (index, height_char) in line.chars().enumerate() {
        let height = height_char.to_string().parse::<i8>().unwrap();
        let highest_north = row_north.map(|vec| {
            let north_tree = vec.get(index).unwrap();
            cmp::max(north_tree.highest_north, north_tree.height)
        }).unwrap_or(-1);
        let new_tree = Tree::new(height, highest_east, highest_north);
        highest_east = cmp::max(highest_east, new_tree.height);
        tree_row.push(new_tree);
    }
    trees.trees.push(tree_row);
    trees
}

fn reduce1(trees: Trees) -> u64 {
    //first we assume all are visible (assuming a rectangular arrangement)
    let width = trees.trees.first().unwrap().len();
    let height = trees.trees.len();

    let mut num_visible: u64 = trees.trees.len() as u64 * width as u64;
    //Now loop through calculating the highestWest and highestSouth as we go
    //When we find a tree that is not visible, decrement the num_visible
    let mut highest_souths: Vec<i8> = Vec::with_capacity(width);
    for _ in 0..width {
        highest_souths.push(-1)
    }

    let mut trees_rows = trees.trees;

    for row_index in (0..height).rev() {
        let mut highest_west: i8 = -1;
        let row = trees_rows.get_mut(row_index).unwrap();

        for col_index in (0..width).rev() {
            let highest_south = highest_souths.get_mut(col_index).unwrap();

            let tree = row.get_mut(col_index).unwrap();
            tree.highest_west = highest_west;
            tree.highest_south = *highest_south;
            highest_west = cmp::max(highest_west, tree.height);
            if tree.height <= tree.highest_north &&
               tree.height <= tree.highest_east &&
               tree.height <= tree.highest_south &&
               tree.height <= tree.highest_west
            {
                num_visible -= 1;
            }

            *highest_south = cmp::max(*highest_south, tree.height)
        }
    }
    num_visible
}

fn reduce2(trees: Trees) -> u64 {
    //brute force it...  meh :(
    let height = trees.trees.len();
    let width = trees.trees.first().unwrap().len();

    let mut max_scenic_score: u64 = 0;

    //Ignore edges, these will always be 0 (i.e. loops from 1..)
    for row in 1..(height-1) {
        for column in 1..(width-1) {

            let viewing_tree = trees.trees.get(row).unwrap().get(column).unwrap();

            let y = row;
            //look east
            let mut view_east = 0;
            for x in (0..column).rev() {
                let visible_tree = trees.trees.get(y).unwrap().get(x).unwrap();
                view_east += 1;
                if visible_tree.height >= viewing_tree.height {
                    break;
                }
            }
            //look west
            let mut view_west = 0;
            for x in (column + 1)..width {
                let visible_tree = trees.trees.get(y).unwrap().get(x).unwrap();
                view_west += 1;
                if visible_tree.height >= viewing_tree.height {
                    break;
                }
            }

            let x = column;
            //look north
            let mut view_north = 0;
            for y in (0..row).rev() {
                let visible_tree = trees.trees.get(y).unwrap().get(x).unwrap();
                view_north += 1;
                if visible_tree.height >= viewing_tree.height {
                    break;
                }
            }
            //look south
            let mut view_south = 0;
            for y in (row+1)..height {
                let visible_tree = trees.trees.get(y).unwrap().get(x).unwrap();
                view_south += 1;
                if visible_tree.height >= viewing_tree.height {
                    break;
                }
            }

            let scenic_score = view_north * view_east * view_south * view_west;
            max_scenic_score = cmp::max(max_scenic_score, scenic_score);
        }
    }

    max_scenic_score
}
