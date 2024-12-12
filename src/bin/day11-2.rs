use std::{collections::HashMap, fs};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(11, false);
    let data = fs::read_to_string(file_path)?;

    let data = data
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let total_blinks = 75;
    let mut solver = Solver::new();
    let mut sum = 0;
    for stone in data {
        sum += solver.solve(stone, total_blinks);
    }
    println!("sum: {sum:?}");

    Ok(())
}

struct Solver {
    cache: HashMap<(u64, usize), u64>,
}

impl Solver {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    fn solve(&mut self, stone: u64, blinks: usize) -> u64 {
        if let Some(cached) = self.cache.get(&(stone, blinks)) {
            return *cached;
        }
        if blinks == 0 {
            return 1;
        }
        let len = if stone == 0 {
            self.solve(1, blinks - 1)
        } else {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let h = 10u64.pow(digits / 2);
                let a = stone / h;
                let b = stone % h;
                self.solve(b, blinks - 1) + self.solve(a, blinks - 1)
            } else {
                self.solve(stone * 2024, blinks - 1)
            }
        };
        self.cache.insert((stone, blinks), len);
        len
    }
}
