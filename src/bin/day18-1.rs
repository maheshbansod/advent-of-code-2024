use std::{
    collections::{HashMap, HashSet},
    fs,
};

use aoc2024::{
    get_input_file,
    maps::{is_out_of_bounds, Coord, Direction},
    MainResult,
};

fn main() -> MainResult {
    let file_path = get_input_file(18, false);
    let data = fs::read_to_string(&file_path)?;
    let limit = 71;
    let data = data
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            (y, x)
            // (x, y)
        })
        .take(1024)
        .collect::<Vec<_>>();
    let walls: HashSet<(usize, usize)> = HashSet::from_iter(data);
    // let grid = Grid::new(data);
    let s_pos = (0, 0);
    let mut to_visit: Vec<(Coord, u32)> = vec![(s_pos, 0)];
    let mut visited = HashMap::new();
    let mut lowest_score = None;
    while let Some((pos, score)) = to_visit.pop() {
        if let Some(lowest_score) = lowest_score {
            if score > lowest_score {
                visited.insert(pos, score);
                continue;
            }
        }
        if pos == (limit - 1, limit - 1) {
            lowest_score = Some(score);
            continue;
        }
        visited.insert(pos, score);
        let possible_moves = Direction::all()
            .iter()
            .filter_map(|d| {
                d.move_pos_checked(pos)
                    .map(|next_pos| (next_pos, score + 1))
            })
            .filter(|&(p, new_score)| {
                !is_out_of_bounds(p, limit, limit)
                    && visited.get(&p).is_none_or(|&s| s > new_score)
                    && !walls.contains(&p)
            })
            .collect::<Vec<_>>();
        for m in possible_moves {
            to_visit.push(m);
        }
    }
    println!("lowest score: {lowest_score:?}");
    Ok(())
}
