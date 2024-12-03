use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let use_test = false;
    let file_path = if use_test {
        "inputs/day2-test"
    } else {
        "inputs/day2"
    };
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let n_safe_reports = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut levels = line.split_whitespace();
            let first = levels.next().unwrap().parse::<u32>().unwrap();
            let second = levels.next();
            if second.is_none() {
                return true;
            }
            let second = second.unwrap().parse::<u32>().unwrap();
            if {
                let diff = second.abs_diff(first);
                diff < 1 || diff > 3
            } {
                return false;
            }
            let direction = get_direction(first, second);
            let mut previous_level = second;
            let mut is_safe = true;
            for level in levels {
                let level = level.parse::<u32>().unwrap();
                if get_direction(previous_level, level) != direction {
                    is_safe = false;
                    break;
                }
                let diff = level.abs_diff(previous_level);
                if diff < 1 || diff > 3 {
                    is_safe = false;
                    break;
                }
                previous_level = level;
            }
            is_safe
        })
        .filter(|&is_safe| is_safe)
        .count();
    println!("{n_safe_reports}");
}

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}
fn get_direction(previous_level: u32, level: u32) -> Direction {
    if previous_level < level {
        Direction::Increasing
    } else {
        Direction::Decreasing
    }
}
