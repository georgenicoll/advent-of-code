use std::io::Error;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use crate::utils;

const FILENAME: &str = "7/input.txt";
//const FILENAME: &str = "7/example.txt";

pub fn _7a() -> Result<u64, Error> {
    utils::process_file(
        FILENAME,
        parse_line,
        Directories::new(),
        accumulate,
        reduce1
    )
}

pub fn _7b() -> Result<u64, Error> {
    utils::process_file(
        FILENAME,
        parse_line,
        Directories::new(),
        accumulate,
        reduce2
    )
}

struct Directories {
    current: Box<Vec<String>>,
    dir_sizes: Box<HashMap<String, u64>>,
    current_level_size: u64,
}

impl Directories {
    pub fn new() -> Directories {
        Directories {
            current: Box::new(Vec::new()),
            dir_sizes: Box::new(HashMap::new()),
            current_level_size: 0,
        }
    }
}

enum Line {
    CD { dir: String },
    LS,
    DIR { _dir: String },
    FILE { name: String, size: u64 }
}

fn parse_line(line: String) -> Option<Line> {
    lazy_static! {
        static ref CD: Regex = Regex::new(r"^\$ cd (.*)$").unwrap();
        static ref LS: Regex = Regex::new(r"^\$ ls.*$").unwrap();
        static ref DIR: Regex = Regex::new(r"^dir (.*)$").unwrap();
        static ref FILE: Regex = Regex::new(r"^(\d+) (.*)$").unwrap();
    }
    None.or_else(||
            CD.captures(&line).map(|cd_caps|
                Line::CD {
                    dir: String::from(cd_caps.get(1).unwrap().as_str())
                }
            )
        ).or_else(||
            LS.captures(&line).map(|_|
                Line::LS
            )
        ).or_else(||
            DIR.captures(&line).map(|dir_caps|
                Line::DIR {
                    _dir: String::from(dir_caps.get(1).unwrap().as_str())
                }
            )
        ).or_else(||
            FILE.captures(&line).map(|file_caps|
                Line::FILE {
                    name: String::from(file_caps.get(2).unwrap().as_str()),
                    size: file_caps.get(1).unwrap().as_str().parse().unwrap()
                }
            )
        )
}

fn accumulate(directories: Directories, maybe_line: Option<Line>) -> Directories {
    match maybe_line {
        Some(Line::CD{ dir }) => cd(directories, dir),
        Some(Line::FILE{ name, size }) => file(directories, name, size),
        Some(Line::LS) => directories,
        Some(Line::DIR{ _dir }) => directories,
        _ => directories,
    }
}

fn cd(mut directories: Directories, dir: String) -> Directories {
    //println!("cd {}", dir);
    let current_dir = build_dir_name(&directories.current);
    //changing directories, stash/increment the size for the current directory
    let current_dir_size = increment_dir_size(&mut directories.dir_sizes, current_dir, directories.current_level_size);
    if dir.eq("..") {
        //moving back up, add this total to the one above and remove this
        directories.current.pop();
        let new_directory = build_dir_name(&directories.current);
        increment_dir_size(&mut directories.dir_sizes, new_directory, current_dir_size);
        directories.current_level_size = 0;
    } else {
        //moving down, add the directory name and reset the current size
        directories.current.push(dir);
        directories.current_level_size = 0;
    }
    directories
}

fn build_dir_name(dirs: &Box<Vec<String>>) -> String {
    let mut s = String::new();
    for dir in dirs.iter() {
        s.push_str("/");
        s.push_str(dir);
    }
    s
}

fn increment_dir_size(dir_sizes: &mut Box<HashMap<String, u64>>, dir_name: String, increment: u64) -> u64 {
    if dir_name.len() > 0 {
        //let dir_name_copy = dir_name.clone();
        let value = dir_sizes
            .entry(dir_name)
            .and_modify(|size| *size += increment)
            .or_insert(increment);
        //println!("Incremented {} by {}, now {}", dir_name_copy, increment, *value);
        0 + *value
    } else {
        0
    }
}

fn file(mut directories: Directories, _name: String, size: u64) -> Directories {
    directories.current_level_size += size;
    directories
}

fn navigate_back_to_top(mut directories: Directories) -> Directories {
    //navigate back to the top level
    while directories.current.len() > 0 {
        directories = cd(directories, String::from(".."));
    }
    directories
}

fn reduce1(mut directories: Directories) -> u64 {
    directories = navigate_back_to_top(directories);
    //Sum up all that are at most 100000
    directories.dir_sizes.values()
        .filter(|v| **v <= 100000)
        .fold(0, |acc, v| acc + *v)
}

fn reduce2(mut directories: Directories) -> u64 {
    directories = navigate_back_to_top(directories);

    const DISK_SPACE: u64 = 70000000;
    const SPACE_NEEDED: u64 = 30000000;
    let root_space = directories.dir_sizes.get("//").unwrap();
    let unused_space: u64 = DISK_SPACE - root_space;
    let need_to_free = SPACE_NEEDED - unused_space;
    println!("root={}, unused={}, need_to_free={}", root_space, unused_space, need_to_free);

    let mut filtered = Vec::from_iter(
        directories.dir_sizes.values()
            .filter(|v| **v >= need_to_free)
            .map(|v| *v )
    );
    filtered.sort();
    *filtered.first().unwrap()
}

