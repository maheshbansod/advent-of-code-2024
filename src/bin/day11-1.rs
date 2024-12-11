use std::fs;

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(11, false);
    let data = fs::read_to_string(file_path)?;

    let mut data = data
        .split_whitespace()
        .map(|c| Some(c.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();

    let total_blinks = 25;
    let mut blinks = 0;
    while blinks < total_blinks {
        let mut stones_to_replace = vec![];
        for stone in &mut data {
            let mut should_replace = false;
            if let Some(stone) = stone {
                if *stone == 0 {
                    *stone = 1;
                } else {
                    let digits = stone.ilog10() + 1;
                    if digits % 2 == 0 {
                        let h = 10u64.pow(digits / 2);
                        should_replace = true;
                        stones_to_replace.push((*stone / h, *stone % h));
                    } else {
                        *stone *= 2024;
                    }
                }
            }
            if should_replace {
                *stone = None;
            }
        }
        for (first_half, second_half) in stones_to_replace {
            data.push(Some(first_half));
            data.push(Some(second_half));
        }
        // println!("{:?}", data);
        blinks += 1;
    }
    let len = data.iter().filter(|d| d.is_some()).count();
    println!("len: {}", len);

    Ok(())
}
