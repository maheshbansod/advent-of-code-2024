use std::{collections::HashSet, fs};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(10, false);
    let data = fs::read_to_string(file_path)?;

    let data = data
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect::<Vec<Vec<_>>>();

    let zeroes = data
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_j, d)| *d == 0)
                .map(move |(j, _d)| (i, j))
        })
        .flatten()
        .collect::<Vec<_>>();
    let nines = data
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_j, d)| *d == 9)
                .map(move |(j, _d)| (i, j))
        })
        .flatten()
        .collect::<Vec<_>>();

    println!("nines: {}", nines.len());

    let mut sum = 0;
    for zero in zeroes {
        let score = trailhead_score(zero, &data, &nines);
        println!("for {zero:?} : {score}");
        sum += score;
    }
    println!("sum: {sum}");
    Ok(())
}

type Node = (usize, usize);

type Data<'a> = &'a [Vec<u8>];

fn trailhead_score(zero: Node, data: &[Vec<u8>], nines: &Vec<Node>) -> usize {
    let mut to_visit = vec![zero];
    let mut visited_nines = HashSet::<Node>::new();
    let mut visited_nodes = HashSet::<Node>::new();
    let mut nine_visited_len = 0;
    while !to_visit.is_empty() {
        let (i, j) = to_visit.pop().unwrap();
        visited_nodes.insert((i, j));
        if data[i][j] == 9 && !visited_nines.contains(&(i, j)) {
            visited_nines.insert((i, j));
            nine_visited_len += 1;
            if nine_visited_len >= nines.len() {
                // all nines are done
                return nine_visited_len;
            }
        };
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        for direction in directions {
            if let Some(next_pos) = get_dir_moved(direction, (i, j), &data) {
                if !visited_nodes.contains(&next_pos) && {
                    let (ni, nj) = next_pos;
                    data[i][j] < data[ni][nj] && data[ni][nj] - data[i][j] == 1
                } {
                    to_visit.push(next_pos);
                }
            }
        }
    }
    return nine_visited_len;
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_dir_moved(direction: Direction, starting_position: Node, data: &Data) -> Option<Node> {
    let (i, j) = starting_position;
    match direction {
        Direction::Up => {
            if i == 0 {
                None
            } else {
                Some((i - 1, j))
            }
        }
        Direction::Left => {
            if j == 0 {
                None
            } else {
                Some((i, j - 1))
            }
        }
        Direction::Down => {
            if i == data.len() - 1 {
                None
            } else {
                Some((i + 1, j))
            }
        }
        Direction::Right => {
            if j == data[0].len() - 1 {
                None
            } else {
                Some((i, j + 1))
            }
        }
    }
}
