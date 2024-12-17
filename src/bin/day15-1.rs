use std::fs;

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(15, false);
    let data = fs::read_to_string(&file_path)?;
    let (init, moves) = data.split_once("\n\n").unwrap();
    let mut grid: Vec<Vec<_>> = init.lines().map(|line| line.chars().collect()).collect();

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
                'O' => {
                    // move O
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
                            'O' => last_o_pos = (nnpi, nnpj),
                            _ => panic!("unexpected"),
                        }
                    }
                    if !did_hit_wall {
                        // then if successful move bot and
                        // clear bot pos
                        grid[bot_pos.0][bot_pos.1] = '.';
                        // move bot
                        grid[npi][npj] = '@';
                        bot_pos = (npi, npj);
                        let (nbi, nbj) = m.direction.move_pos(last_o_pos);
                        grid[nbi][nbj] = 'O';
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
    }

    let sum: usize = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, c)| *c == 'O')
                .map(|(j, _)| i * 100 + j)
                .collect::<Vec<_>>()
        })
        .flatten()
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
