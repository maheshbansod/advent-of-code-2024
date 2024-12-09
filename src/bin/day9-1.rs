use std::fs;

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(9, false);
    let data = fs::read_to_string(file_path)?;
    let data = data.lines().next().unwrap();
    let mut data = data
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();

    let mut pos = 0u64;
    let mut sum = 0u64;
    let mut is_free_space = false;
    let mut len = data.len();

    let mut i = 0;
    while i < len {
        let d = data[i];
        if is_free_space {
            // free
            let mut free = d;
            if free > data[len - 1] {
                loop {
                    let last = data[len - 1];
                    let file_id = (len - 1) as u64 / 2;
                    for j in pos..pos + last {
                        let product = file_id * j;
                        sum += product;
                    }
                    data[len - 1] = 0;
                    free -= last;
                    len -= 2;
                    pos += last;
                    if len <= 2 {
                        break;
                    }
                    if free >= data[len - 1] {
                        // continue doing what we doing
                    } else if free != 0 {
                        data[len - 1] -= free;
                        let file_id = (len - 1) as u64 / 2;
                        for j in pos..pos + free {
                            // sum += file_id * j;
                            let product = file_id * j;
                            sum += product;
                        }
                        pos += free;
                        break;
                    } else {
                        break;
                    }
                }
                data[i] = 0;
            } else {
                data[len - 1] -= free;
                let file_id = (len - 1) as u64 / 2;
                for j in pos..pos + free {
                    let product = file_id * j;
                    sum += product;
                }
                pos += free;
                data[i] = 0;
            }
        } else {
            let file_id = i as u64 / 2;
            for j in pos..pos + d {
                let product = file_id * j;
                sum += product;
            }
            data[i] = 0;
            pos += d;
        }
        is_free_space = !is_free_space;

        i += 1;
    }

    println!("sum: {sum}");

    Ok(())
}
