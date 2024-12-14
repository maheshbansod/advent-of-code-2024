use std::fs;

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(14, false);
    let data = fs::read_to_string(&file_path)?;
    let data = data
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (_, p) = p.split_once('=').unwrap();
            let (px, py) = p.split_once(",").unwrap();
            let px: i32 = px.parse().unwrap();
            let py: i32 = py.parse().unwrap();
            let (_, v) = v.split_once('=').unwrap();
            let (vx, vy) = v.split_once(",").unwrap();
            let vx: i32 = vx.parse().unwrap();
            let vy: i32 = vy.parse().unwrap();

            let total_seconds = 100;
            let grid_height = 103;
            let grid_width = 101;
            // let grid_width = 11;
            // let grid_height = 7;

            let new_x = px + vx * total_seconds;
            let new_y = py + vy * total_seconds;
            let new_x = new_x % grid_width;
            let new_y = new_y % grid_height;
            let new_x = if new_x < 0 { grid_width + new_x } else { new_x };
            let new_y = if new_y < 0 {
                grid_height + new_y
            } else {
                new_y
            };
            let xmid = grid_width / 2;
            let ymid = grid_height / 2;
            let q = if new_x < xmid {
                if new_y < ymid {
                    0
                } else if new_y > ymid {
                    2
                } else {
                    4 // represents mid
                }
            } else if new_x > xmid {
                if new_y < ymid {
                    1
                } else if new_y > ymid {
                    3
                } else {
                    4
                }
            } else {
                4
            };
            (new_x, new_y, q)
        })
        .filter(|a| a.2 != 4);
    let mut chunks = [0, 0, 0, 0];
    for (_, _, q) in data {
        chunks[q] += 1;
    }
    println!("{chunks:?}");
    let product = chunks[0] * chunks[1] * chunks[2] * chunks[3];
    println!("{product}");
    Ok(())
}

// struct Bot {
//     pos: Coord,
//     v: Coord,
// }
//
// type Coord = (i32, i32);
