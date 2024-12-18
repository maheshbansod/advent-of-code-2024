use std::{collections::HashSet, fs};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(15, false);
    let data = fs::read_to_string(&file_path)?;
    let (init, moves) = data.split_once("\n\n").unwrap();
    let mut grid: Vec<Vec<_>> = init
        .lines()
        .map(|line| {
            let line = line
                .chars()
                .flat_map(|c| match c {
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    c => [c, c],
                })
                .collect::<Vec<_>>();

            println!("l: {line:?}");
            line
        })
        .collect();
    for row in grid.iter() {
        for c in row.iter() {
            print!("{c}");
        }
        println!()
    }

    let moves = moves.chars().filter_map(|c| match c {
        '>' => Some(Direction::Right),
        '<' => Some(Direction::Left),
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        _ => None,
    });
    let moves = {
        let mut previous: Option<Move> = None;
        let mut results = vec![];
        for m in moves {
            let mut should_replace = true;
            if let Some(previous_value) = &mut previous {
                if m == previous_value.direction {
                    previous_value.amount += 1;
                    should_replace = false;
                } else {
                    results.push(previous_value.clone());
                }
            }
            if should_replace {
                previous = Some(Move {
                    direction: m,
                    amount: 1,
                });
            }
        }
        if let Some(previous) = previous {
            results.push(previous);
        }
        results
    };

    let mut bot_pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|&(_j, c)| *c == '@')
                .map(|(j, _c)| (i, j))
        })
        .unwrap();

    for m in moves {
        for _ in 0..m.amount {
            let (npi, npj) = m.direction.move_pos(bot_pos);
            match grid[npi][npj] {
                '.' => {
                    // clear bot pos
                    grid[bot_pos.0][bot_pos.1] = '.';
                    // move bot
                    grid[npi][npj] = '@';
                    bot_pos = (npi, npj);
                }
                ']' | '[' => {
                    // move wall
                    let mut last_o_pos = (npi, npj);
                    let mut did_hit_wall = false;
                    loop {
                        let (nnpi, nnpj) = m.direction.move_pos(last_o_pos);
                        match grid[nnpi][nnpj] {
                            '.' => break,
                            '#' => {
                                did_hit_wall = true;
                                break;
                            }
                            ']' | '[' => last_o_pos = (nnpi, nnpj),
                            c => panic!("unexpected {c}"),
                        }
                    }
                    if !did_hit_wall {
                        match m.direction {
                            Direction::Up | Direction::Down => {
                                // need to move walls that connect
                                bot_pos = move_walls_vertical(m.direction, bot_pos, &mut grid);
                            }
                            Direction::Left | Direction::Right => {
                                // kinda simple case - just shift
                                // everything
                                let mut pos_to_move = bot_pos;
                                let mut previous_char = '.';
                                loop {
                                    // at this point we know it
                                    // didn't hit wall so let's
                                    // just shift till empty
                                    // space
                                    let (i, j) = pos_to_move;
                                    std::mem::swap(&mut grid[i][j], &mut previous_char);
                                    if previous_char == '.' {
                                        break;
                                    }
                                    pos_to_move = m.direction.move_pos(pos_to_move);
                                }
                                bot_pos = (npi, npj);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        // println!("after move {m:?}");
        // for row in grid.iter() {
        //     for c in row.iter() {
        //         print!("{c}");
        //     }
        //     println!()
        // }
        // thread::sleep(Duration::from_secs(1));
    }

    let sum: usize = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, c)| *c == '[')
                .map(|(j, _)| i * 100 + j)
                .collect::<Vec<_>>()
        })
        .sum();
    println!("sum: {sum}");

    Ok(())
}

#[derive(Clone, Debug)]
struct Move {
    direction: Direction,
    amount: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn move_pos(&self, next_pos: Coord) -> Coord {
        let (i, j) = next_pos;
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }
}

type Coord = (usize, usize);

/// Returns bot pos and updates grid
fn move_walls_vertical(
    direction: Direction,
    starting_position: Coord,
    grid: &mut [Vec<char>],
) -> Coord {
    let mut to_moves = vec![];
    let mut to_checks = vec![starting_position];
    let mut leave_dot = HashSet::new();
    leave_dot.insert(starting_position);
    while let Some(position) = to_checks.pop() {
        let (npi, npj) = direction.move_pos(position);
        match grid[npi][npj] {
            '#' => return starting_position,
            '[' => {
                to_checks.push((npi, npj));
                if leave_dot.contains(&(npi, npj)) {
                    leave_dot.remove(&(npi, npj));
                }
                to_checks.push((npi, npj + 1));
                leave_dot.insert((npi, npj + 1));
            }
            ']' => {
                to_checks.push((npi, npj));
                if leave_dot.contains(&(npi, npj)) {
                    leave_dot.remove(&(npi, npj));
                }
                to_checks.push((npi, npj - 1));
                leave_dot.insert((npi, npj - 1));
            }
            _ => {}
        };
        to_moves.push((position, grid[position.0][position.1]));
    }
    for (p, c) in to_moves {
        let (i, j) = direction.move_pos(p);
        leave_dot.remove(&(i, j));
        grid[i][j] = c;
    }
    for (i, j) in leave_dot {
        grid[i][j] = '.';
    }
    direction.move_pos(starting_position)
}
