use std::{collections::HashSet, fs};

use aoc2024::{
    get_input_file,
    maps::{is_out_of_bounds, Coord, Direction},
    MainResult,
};

fn main() -> MainResult {
    let file_path = get_input_file(18, false);
    let data = fs::read_to_string(&file_path)?;
    let limit = 71;
    // let limit = 7;
    let data = data
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            (y, x)
            // (x, y)
        })
        .collect::<Vec<_>>();
    for i in 1024..data.len() {
        let walls: HashSet<(usize, usize)> = HashSet::from_iter(data[..i].iter().copied());
        // let grid = Grid::new(data);
        let s_pos = (0, 0);
        let mut to_visit: Vec<Coord> = vec![s_pos];
        let mut visited = HashSet::new();
        let mut reached = false;
        while let Some(pos) = to_visit.pop() {
            if visited.contains(&pos) {
                continue;
            }
            if pos == (limit - 1, limit - 1) {
                reached = true;
                break;
            }
            visited.insert(pos);
            let possible_moves = Direction::all()
                .iter()
                .filter_map(|d| d.move_pos_checked(pos))
                .filter(|&p| {
                    !is_out_of_bounds(p, limit, limit)
                        && !visited.contains(&p)
                        && !walls.contains(&p)
                })
                .collect::<Vec<_>>();
            for m in possible_moves {
                to_visit.push(m);
            }
        }
        if !reached {
            let last_coord = data[i - 1];
            println!("Couldnt reach the end boss");
            println!("(0:{i}): {},{}", last_coord.1, last_coord.0);
            break;
        }
    }
    Ok(())
}
