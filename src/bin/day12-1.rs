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
            let mut fence_perimeter = 0;
            while let Some((pi, pj)) = to_visit.pop() {
                if visited.contains(&(pi, pj)) {
                    continue;
                }
                visited.insert((pi, pj));
                fence_area += 1;
                let neighbours = get_neighbours(pi, pj);
                for n in neighbours {
                    if let Some((ni, nj)) = n {
                        if let Some(nd) = data.get(ni).map(|row| row.get(nj)).flatten() {
                            if nd == region_head {
                                if !visited.contains(&(ni, nj)) {
                                    to_visit.push((ni, nj));
                                }
                                continue;
                            }
                        }
                    }
                    fence_perimeter += 1;
                }
            }
            println!("Region {region_head} a: {fence_area}, p: {fence_perimeter}");
            prices.push(fence_area * fence_perimeter);
        }
    }
    let sum: u32 = prices.iter().sum();
    println!("{sum}");
    Ok(())
}

/// Returns the neighbours of a coordinate
///
/// [[Up, Right, Left, Down]]
const fn get_neighbours(i: usize, j: usize) -> [Option<(usize, usize)>; 4] {
    [
        if i > 0 { Some((i - 1, j)) } else { None },
        Some((i, j + 1)),
        if j > 0 { Some((i, j - 1)) } else { None },
        Some((i + 1, j)),
    ]
}
