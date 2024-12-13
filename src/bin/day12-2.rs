use std::{collections::HashSet, fs};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(12, false);
    let data = fs::read_to_string(&file_path)?;
    let data = data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut visited = HashSet::new();
    let mut prices = vec![];
    for (i, row) in data.iter().enumerate() {
        for (j, region_head) in row.iter().enumerate() {
            if visited.contains(&(i, j)) {
                continue;
            }
            let mut to_visit = Vec::new();
            to_visit.push((i, j));
            let mut fence_area = 0;
            let mut fence_havers = HashSet::new();
            while !to_visit.is_empty() {
                let mut next_visit = vec![];
                for &(pi, pj) in to_visit.iter() {
                    if visited.contains(&(pi, pj)) {
                        continue;
                    }
                    visited.insert((pi, pj));
                    fence_area += 1;
                    let neighbours = get_neighbours(pi, pj);
                    for n in neighbours {
                        let (direction, n) = n;
                        if let Some((ni, nj)) = n {
                            if let Some(nd) = data.get(ni).map(|row| row.get(nj)).flatten() {
                                if nd == region_head {
                                    if !visited.contains(&(ni, nj)) {
                                        next_visit.push((ni, nj));
                                    }
                                    continue;
                                }
                            }
                        }
                        fence_havers.insert((direction, (pi, pj)));
                    }
                }
                to_visit = next_visit;
            }
            let mut sum = 0;
            let directions = [Direction::Up, Direction::Down];
            for direction in directions {
                let mut horizontal_fenced = fence_havers
                    .iter()
                    .filter(|f| f.0 == direction)
                    .collect::<Vec<_>>();
                horizontal_fenced.sort_by(|a, b| {
                    (a.1 .0 * data[0].len() + a.1 .1).cmp(&(b.1 .0 * data[0].len() + b.1 .1))
                });
                let chunks = horizontal_fenced
                    .chunk_by(|&(ad, ap), &(bd, bp)| {
                        ad == bd && ap.0 == bp.0 && ap.1.abs_diff(bp.1) == 1
                    })
                    .collect::<Vec<_>>();
                sum += chunks.len();
            }
            let directions = [Direction::Left, Direction::Right];
            for direction in directions {
                let mut vertical_fenced = fence_havers
                    .iter()
                    .filter(|f| f.0 == direction)
                    .collect::<Vec<_>>();
                vertical_fenced.sort_by(|a, b| {
                    (a.1 .1 * data.len() + a.1 .0).cmp(&(b.1 .1 * data[0].len() + b.1 .0))
                });
                let chunks = vertical_fenced
                    .chunk_by(|&(ad, ap), &(bd, bp)| {
                        ad == bd && ap.1 == bp.1 && ap.0.abs_diff(bp.0) == 1
                    })
                    .collect::<Vec<_>>();
                sum += chunks.len();
            }
            let fence_sides = sum;

            prices.push(fence_area * fence_sides);
        }
    }
    let sum: usize = prices.iter().sum();
    println!("{sum}");
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn _adjacent(&self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
    const fn pos(&self, i: usize, j: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if i > 0 {
                    Some((i - 1, j))
                } else {
                    None
                }
            }
            Direction::Down => Some((i + 1, j)),
            Direction::Left => {
                if j > 0 {
                    Some((i, j - 1))
                } else {
                    None
                }
            }
            Direction::Right => Some((i, j + 1)),
        }
    }
}

/// Returns the neighbours of a coordinate
///
/// [[Up, Right, Left, Down]]
const fn get_neighbours(i: usize, j: usize) -> [(Direction, Option<(usize, usize)>); 4] {
    [
        (Direction::Up, Direction::Up.pos(i, j)),
        (Direction::Left, Direction::Left.pos(i, j)),
        (Direction::Right, Direction::Right.pos(i, j)),
        (Direction::Down, Direction::Down.pos(i, j)),
    ]
}
