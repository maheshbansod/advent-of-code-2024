use std::{
    collections::{HashMap, HashSet},
    fs,
};

use aoc2024::{
    get_input_file,
    maps::{Coord, Direction, Grid},
    MainResult,
};

fn main() -> MainResult {
    let file_path = get_input_file(16, false);
    let data = fs::read_to_string(&file_path)?;
    let data = data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let grid = Grid::new(data);
    let s_pos = grid.find_position('S').unwrap();
    let mut to_visit: Vec<(Coord, Direction, u32, Vec<Coord>)> =
        vec![(s_pos, Direction::Right, 0, vec![s_pos])];
    let mut visited = HashMap::new();
    let mut lowest_score = None;
    let mut seats = HashSet::new();
    while let Some((pos, direction, score, way)) = to_visit.pop() {
        if let Some(lowest_score) = lowest_score {
            if score > lowest_score {
                // println!("ignoring path at {pos:?}. {score}");
                visited.insert((pos, direction), score);
                continue;
            }
        }
        if grid.at(pos) == 'E' {
            // println!("FOUND STORING!! {score}");
            if let Some(lowest_score) = lowest_score {
                if lowest_score > score {
                    seats = HashSet::new();
                }
            }
            lowest_score = Some(score);
            for pos in way {
                seats.insert(pos);
            }
            continue;
        }
        visited.insert((pos, direction), score);
        // println!("visiting: {pos:?}");

        let possible_moves = Direction::all()
            .iter()
            .map(|d| {
                let next_pos = d.move_pos(pos);
                (
                    next_pos,
                    *d,
                    score + 1 + d.diff(&direction) as u32 * 1000,
                    {
                        let mut way = way.clone();
                        way.push(next_pos);
                        way
                    },
                )
            })
            .filter(|&(p, d, new_score, _)| {
                grid.at_checked(p).is_some_and(|c| {
                    let result = visited.get(&(p, d)).is_none_or(|&s| s >= new_score) && *c != '#';
                    result
                })
            })
            .collect::<Vec<_>>();
        for m in possible_moves {
            to_visit.push(m);
        }
    }
    println!("seats: {}", seats.len());
    println!("lowest score: {lowest_score:?}");
    Ok(())
}
