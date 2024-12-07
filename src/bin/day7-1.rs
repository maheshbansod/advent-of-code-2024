use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(7, false);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap());
    let mut sum = 0;
    for line in lines {
        let (result, rest) = line.split_once(": ").unwrap();
        let result = result.parse::<u64>().unwrap();
        let numbers = rest
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            // .rev()
            .collect::<Vec<_>>();
        if traverse(&numbers, result) {
            // println!("yayy {numbers:?}, {result}");
            sum += result;
        } else {
            // println!("nayy {numbers:?}, {result}");
        }
    }
    println!("sum: {sum}");
    Ok(())
}

fn traverse(numbers: &[u64], expected_result: u64) -> bool {
    let last = numbers.last().unwrap();
    let numbers = &numbers[0..numbers.len() - 1];

    if let Some(remaining) = expected_result.checked_sub(*last) {
        if numbers.len() == 1 && remaining == numbers[0] {
            return true;
        }
        if remaining > 0 && numbers.len() >= 2 && traverse(numbers, remaining) {
            return true;
        }
    }

    if expected_result % last == 0 {
        let remaining = expected_result / last;
        if numbers.len() == 1 && remaining == numbers[0] {
            return true;
        }
        if remaining >= 1 && numbers.len() >= 2 && traverse(numbers, remaining) {
            return true;
        }
    }

    false
}
