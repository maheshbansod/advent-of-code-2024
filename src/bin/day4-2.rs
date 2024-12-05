use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let use_test = false;
    let file_path = if use_test {
        "inputs/day4-test"
    } else {
        "inputs/day4"
    };
    let data = fs::read_to_string(file_path)?;
    let data = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut xmas_count = 0;
    for (n_line, line) in data.iter().enumerate() {
        for (j, _c) in line.iter().enumerate() {
            xmas_count += count_x_mas_at(&data, n_line, j);
        }
    }

    println!("count: {xmas_count}");
    Ok(())
}

fn count_x_mas_at(data: &Vec<Vec<char>>, start_i: usize, start_j: usize) -> usize {
    if data[start_i][start_j] != 'A' {
        return 0;
    }
    let row_len = data[0].len();
    let total_lines = data.len();
    if start_i == 0 || start_i + 1 >= total_lines || start_j == 0 || start_j + 1 >= row_len {
        return 0;
    }
    let top_left = data[start_i - 1][start_j - 1];
    let top_right = data[start_i - 1][start_j + 1];

    let bottom_left = data[start_i + 1][start_j - 1];
    let bottom_right = data[start_i + 1][start_j + 1];

    if !(top_left == 'M' && bottom_right == 'S' || (top_left == 'S' && bottom_right == 'M')) {
        return 0;
    }
    if !(top_right == 'M' && bottom_left == 'S' || (top_right == 'S' && bottom_left == 'M')) {
        return 0;
    }

    return 1;
}
