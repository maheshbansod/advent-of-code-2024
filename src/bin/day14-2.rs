use std::fs;

use aoc2024::{get_input_file, MainResult};

const GRID_HEIGHT: usize = 103;
const GRID_WIDTH: usize = 101;
fn main() -> MainResult {
    let file_path = get_input_file(14, false);
    let data = fs::read_to_string(&file_path)?;
    let mut data = data
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

            Bot {
                pos: (px, py),
                v: (vx, vy),
            }
        })
        .collect::<Vec<_>>();
    let mut counter = 0;
    let total_seconds = 20000;
    loop {
        if counter > total_seconds {
            break;
        }
        counter += 1;
        let mut grid = [[0; GRID_WIDTH]; GRID_HEIGHT];
        for bot in &data {
            let (x, y) = bot.pos;
            grid[y as usize][x as usize] = 1;
        }
        // render
        let mut this_is_it = false;
        for (i, row) in grid.iter().enumerate() {
            let mut previous = 0;
            let mut match_count = 0;
            for d in row {
                if *d == 0 {
                    print!(".");
                } else {
                    print!("O");
                }
                if previous == *d && previous != 0 {
                    match_count += 1;
                }
                previous = *d;
            }
            if match_count >= 25 {
                this_is_it = true;
            }
            print!("{counter} {i}\n");
        }
        if this_is_it {
            panic!("is this it!!!");
        }
        // update
        for bot in &mut data {
            let (x, y) = bot.pos;
            let (vx, vy) = bot.v;
            let x = x + vx;
            let y = y + vy;
            let y = y % GRID_HEIGHT as i32;
            let y = if y < 0 { GRID_HEIGHT as i32 + y } else { y };
            let x = x % GRID_WIDTH as i32;
            let x = if x < 0 { GRID_WIDTH as i32 + x } else { x };
            bot.pos = (x, y);
        }
    }
    Ok(())
}

struct Bot {
    pos: Coord,
    v: Coord,
}

type Coord = (i32, i32);
