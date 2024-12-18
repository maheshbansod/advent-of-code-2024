use std::{
    collections::HashMap,
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
    let mut to_visit: Vec<(Coord, Direction, u32)> = vec![(s_pos, Direction::Right, 0)];
    let mut visited = HashMap::new();
    let mut lowest_score = None;
    while let Some((pos, direction, score)) = to_visit.pop() {
        if let Some(lowest_score) = lowest_score {
            if score > lowest_score {
                // println!("ignoring path at {pos:?}. {score}");
                visited.insert(pos, score);
                continue;
            }
        }
        if grid.at(pos) == 'E' {
            // println!("FOUND STORING!! {score}");
            lowest_score = Some(score);
            continue;
        }
        visited.insert(pos, score);
        // println!("visiting: {pos:?}");

        let possible_moves = Direction::all()
            .iter()
            .map(|d| {
                (
                    d.move_pos(pos),
                    *d,
                    score + 1 + d.diff(&direction) as u32 * 1000,
                )
            })
            .filter(|(p, _, new_score)| {
                grid.at_checked(*p)
                    .is_some_and(|c| visited.get(p).is_none_or(|s| s >= new_score) && *c != '#')
            })
            .collect::<Vec<_>>();
        // println!("pos: {possible_moves:?}");
        for m in possible_moves {
            to_visit.push(m);
        }
    }
    println!("lowest score: {lowest_score:?}");
    Ok(())
}
