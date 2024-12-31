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
    let file_input = get_input_file(20, false);
    let data = fs::read_to_string(&file_input)?;
    let data = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut grid = Grid::new(data);
    let start_position = grid.find_position('S').unwrap();
    let end_position = grid.find_position('E').unwrap();

    *grid.at_mut(start_position) = '.';
    *grid.at_mut(end_position) = '.';
    let grid = grid;
    println!("Read the grid and identified start and end positions.");

    let mut used_coord = HashSet::new();
    let mut total_coords = HashSet::new();
    let path = get_path_between(&grid, start_position, end_position);
    let original_path_len = path.len();
    println!("Original path stored. Size: {original_path_len}");
    let mut saved_count = HashMap::new();
    for (i, start_pos) in path.iter().enumerate() {
        let mut path_finder = PathFinder::new(start_pos.clone());
        println!(
            "Checking from {}th starting position/{original_path_len} ({}%)",
            i + 1,
            (i + 1) * 100 / original_path_len
        );
        for (j, end_pos) in path.iter().enumerate().skip(i + 1) {
            if let Some(new_diff_len) = path_finder.get_shortest_path_len(*end_pos) {
                // in this case, i need to see how much does this path save

                // the length from `start_pos` to `end_pos`
                let original_diff_len = j - i;
                let new_path_len = original_path_len - original_diff_len + new_diff_len as usize;
                used_coord.insert((i, j));
                if new_path_len < original_path_len {
                    let saved = original_path_len - new_path_len;
                    // if saved == 66 {
                    //     println!("saved_ba{start_pos:?}e{end_pos:?},ij:<{i},{j}>,ndl:{new_diff_len},odl:{original_diff_len}");
                    //     println!("npl:{new_path_len},opl:{original_path_len}");
                    //     // pause();
                    // }
                    saved_count
                        .entry(saved)
                        .and_modify(|s| *s += 1)
                        .or_insert(1);
                };
            }
            total_coords.insert((i, j));
        }
    }

    println!("saved: {:#?}", saved_count);
    let saved_limit = 100;
    let mut sum = 0;
    for (saved_secs, count) in saved_count.iter() {
        if *saved_secs >= saved_limit {
            sum += count;
            println!("There are {count} cheats that save {saved_secs} seconds");
        };
    }
    println!("sum: {sum}");
    Ok(())
}

struct PathFinder {
    start_position: Coord,
    path_limit: u32,
}
impl PathFinder {
    fn new(start_position: Coord) -> Self {
        Self {
            start_position,
            path_limit: 20,
        }
    }
    fn get_shortest_path_len(&mut self, end_position: Coord) -> Option<u32> {
        let (si, sj) = self.start_position;
        let (ei, ej) = end_position;
        let idiff = si.abs_diff(ei);
        let jdiff = sj.abs_diff(ej);
        let total_picoseconds = idiff as u32 + jdiff as u32;
        if total_picoseconds <= self.path_limit {
            return Some(total_picoseconds);
        } else {
            return None;
        }
    }
}

/// Return the path between the start position and end position
/// Assumes that there exists exactly one path between start and end
fn get_path_between(grid: &Grid<char>, start_position: Coord, end_position: Coord) -> Vec<Coord> {
    let mut path = vec![];
    path.push(start_position);
    let mut pos = start_position;
    let mut previous: Option<Coord> = None;
    loop {
        if let Some(moved_pos) = Direction::all().iter().find_map(|&d| {
            d.move_pos_checked(pos).filter(|&pos| {
                matches!(grid.at_checked(pos), Some('.'))
                    && previous.is_none_or(|previous| previous != pos)
            })
        }) {
            previous = Some(pos);
            pos = moved_pos;
            path.push(moved_pos);
            if pos == end_position {
                break;
            }
        } else {
            break;
        }
    }
    path
}
