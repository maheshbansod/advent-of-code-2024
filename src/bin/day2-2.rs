use core::panic;
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
            enum DMode {
                Available,
                Active((u32, Option<u32>)),
                Used,
            }
            let mut dampener_mode = DMode::Available;
            let first = levels.next().unwrap().parse::<u32>().unwrap();
            let second = levels.next().unwrap().parse::<u32>().unwrap();
            let mut levels_dir = levels.clone();
            let mut previous_level = second;
            let mut increasing_count = if get_direction(first, second) == Direction::Increasing {
                1
            } else {
                -1
            };
            while let Some(level) = levels_dir.next() {
                let level = level.parse::<u32>().unwrap();
                if get_direction(previous_level, level) == Direction::Increasing {
                    increasing_count += 1;
                } else {
                    increasing_count -= 1;
                }
                if increasing_count > 2 || increasing_count < -2 {
                    break;
                }
                previous_level = level;
            }
            let direction = if increasing_count < 1 {
                Direction::Decreasing
            } else {
                Direction::Increasing
            };
            let mut pre_previous_level = first;
            let mut previous_level = second;
            if !is_level_pair_safe((first, second), &direction) {
                let third = levels.next();
                if third.is_none() {
                    return true;
                }
                let third = third.unwrap().parse::<u32>().unwrap();
                if is_level_pair_safe((first, third), &direction) {
                    previous_level = third;
                } else if is_level_pair_safe((second, third), &direction) {
                    pre_previous_level = second;
                    previous_level = third;
                } else {
                    return false;
                }
                dampener_mode = DMode::Used;
            }
            for level in levels {
                let level = level.parse::<u32>().unwrap();
                if let DMode::Active((previous1, previous2)) = dampener_mode {
                    if !is_level_pair_safe((previous1, level), &direction) {
                        if let Some(previous2) = previous2 {
                            if !is_level_pair_safe((previous2, level), &direction) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    dampener_mode = DMode::Used;
                } else if !is_level_pair_safe((previous_level, level), &direction) {
                    match dampener_mode {
                        DMode::Available => {
                            if is_level_pair_safe((pre_previous_level, level), &direction) {
                                dampener_mode = DMode::Active((level, Some(previous_level)));
                            } else {
                                dampener_mode = DMode::Active((previous_level, None));
                            }
                            // we set this to pre-previous so that the
                            // pre-previous level doesn't change
                            previous_level = pre_previous_level;
                        }
                        DMode::Used => {
                            return false;
                        }
                        DMode::Active(_) => {
                            panic!("Unexpected");
                        }
                    }
                }
                pre_previous_level = previous_level;
                previous_level = level;
            }
            true
        })
        .filter(|&is_safe| is_safe)
        .count();
    println!("{n_safe_reports}");
}

fn is_level_pair_safe(levels: (u32, u32), direction: &Direction) -> bool {
    if get_direction(levels.0, levels.1) != *direction {
        return false;
    }
    let diff = levels.1.abs_diff(levels.0);
    !(diff < 1 || diff > 3)
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
