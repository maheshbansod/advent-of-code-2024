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
            xmas_count += count_xmas_at(&data, n_line, j);
        }
    }

    println!("count: {xmas_count}");
    Ok(())
}

fn count_xmas_at(data: &[Vec<char>], start_i: usize, start_j: usize) -> usize {
    const WORD: &[char] = &['X', 'M', 'A', 'S'];
    if data[start_i][start_j] != WORD[0] {
        return 0;
    }
    let word_len = WORD.len();
    let row_len = data[0].len();
    let total_lines = data.len();
    let mut wi = 0;
    let mut horizontal_forward = true;
    let mut horizontal_backward = true;
    let mut diagonal_forward_down = true;
    let mut diagonal_backward_down = true;
    let mut diagonal_forward_up = true;
    let mut diagonal_backward_up = true;
    let mut vertical_up = true;
    let mut vertical_down = true;

    while wi < word_len {
        if horizontal_forward
            && (wi + start_j >= row_len || data[start_i][wi + start_j] != WORD[wi])
        {
            horizontal_forward = false;
        }
        if horizontal_backward && (start_j < wi || data[start_i][start_j - wi] != WORD[wi]) {
            horizontal_backward = false;
        }
        if vertical_down && (wi + start_i >= total_lines || data[start_i + wi][start_j] != WORD[wi])
        {
            vertical_down = false;
        }
        if vertical_up && (start_i < wi || data[start_i - wi][start_j] != WORD[wi]) {
            vertical_up = false;
        }
        if diagonal_forward_down
            && (start_j + wi >= row_len
                || start_i + wi >= total_lines
                || data[start_i + wi][start_j + wi] != WORD[wi])
        {
            diagonal_forward_down = false;
        }
        if diagonal_backward_down
            && (start_j < wi
                || start_i + wi >= total_lines
                || data[start_i + wi][start_j - wi] != WORD[wi])
        {
            diagonal_backward_down = false;
        }
        if diagonal_forward_up
            && (start_j + wi >= row_len
                || start_i < wi
                || data[start_i - wi][start_j + wi] != WORD[wi])
        {
            diagonal_forward_up = false;
        }
        if diagonal_backward_up
            && (start_j < wi || start_i < wi || data[start_i - wi][start_j - wi] != WORD[wi])
        {
            diagonal_backward_up = false;
        }
        wi += 1;
    }

    [
        horizontal_backward,
        horizontal_forward,
        vertical_up,
        vertical_down,
        diagonal_forward_down,
        diagonal_forward_up,
        diagonal_backward_up,
        diagonal_backward_down,
    ]
    .iter()
    .filter(|i| **i)
    .count()
}
