use std::{collections::HashMap, fs};

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

    let grid = Grid::new(data);
    let start_position = grid.find_position('S').unwrap();
    let end_position = grid.find_position('E').unwrap();

    let mut finder = PathFinder::new(grid.clone(), end_position);
    let original_len = finder.shortest_path_len(&start_position).unwrap();

    println!("shortest_path: {:?}", original_len);

    let v = grid
        .find_all_positions('#')
        .filter(|wp| !grid.is_edge(wp))
        .filter_map(|wall_position| {
            let mut grid = grid.clone();
            let wall = grid.at_mut(wall_position);
            *wall = '.'; // remove the wall
            let mut finder = PathFinder::new(grid, end_position);
            let result = finder
                .shortest_path_len(&start_position)
                .map(|shortest_len| (wall_position, original_len - shortest_len));
            result
        })
        .filter(|(_, saved)| *saved >= 100)
        .count();
    // let mut saved_map = HashMap::new();
    // for (_pos, saved) in v {
    //     saved_map
    //         .entry(saved)
    //         .and_modify(|saved| *saved += 1)
    //         .or_insert(1);
    // }
    // println!("{saved_map:?}");
    println!("count: {v}");

    Ok(())
}

struct PathFinder {
    grid: Grid<char>,
    end_position: Coord,
}

impl PathFinder {
    pub fn new(grid: Grid<char>, end_position: Coord) -> Self {
        Self { grid, end_position }
    }

    fn shortest_path_len(&mut self, start_position: &Coord) -> Option<usize> {
        // let cache = HashMap::new();
        let mut visited = HashMap::new();
        let end_position = self.end_position.clone();

        // let mut to_visit = vec![(*start_position, so_far)];
        let mut stack = vec![StackElem {
            pos: *start_position,
            so_far: 0,
            data: StackElemData::Encountered,
        }];

        while let Some(StackElem { pos, data, so_far }) = stack.pop() {
            // println!("{pos:?}: {data:?}. steps: {so_far}");
            match data {
                StackElemData::Finalized {
                    min_path: min_path_found,
                } => {
                    if min_path_found.is_none() {
                        continue;
                    }
                    let min_path_found = min_path_found.unwrap();
                    if let Some(StackElem {
                        pos,
                        so_far,
                        data: StackElemData::Visiting { i, min_path },
                    }) = stack.pop()
                    {
                        let mut nothing_to_do = false;
                        if let Some(previous_min) = min_path {
                            if previous_min < min_path_found {
                                nothing_to_do = true;
                            }
                        }
                        if !nothing_to_do {
                            stack.push(StackElem {
                                pos,
                                so_far,
                                data: StackElemData::Visiting {
                                    i,
                                    min_path: Some(min_path_found),
                                },
                            });
                        }
                    } else {
                        return Some(min_path_found);
                    }
                }
                StackElemData::Encountered => {
                    // if visited.contains_key(&pos) {
                    //     continue;
                    // }
                    if pos == end_position {
                        stack.push(StackElem {
                            pos,
                            so_far,
                            data: StackElemData::Finalized {
                                min_path: Some(so_far),
                            },
                        });
                    } else {
                        stack.push(StackElem {
                            pos,
                            so_far,
                            data: StackElemData::Visiting {
                                i: 0,
                                min_path: None,
                            },
                        });
                    }
                    visited.insert(pos, so_far);
                }
                StackElemData::Visiting { i, min_path } => {
                    if let Some(to_visit) = Direction::all()
                        .iter()
                        .map(|&d| d.move_pos(pos))
                        .filter(|pos| {
                            self.grid.at(*pos) != '#'
                                && visited
                                    .get(pos)
                                    .is_none_or(|&previous_count| previous_count >= so_far + 1)
                        })
                        .skip(i)
                        .next()
                    {
                        stack.push(StackElem {
                            pos,
                            so_far,
                            data: StackElemData::Visiting { i: i + 1, min_path },
                        });
                        stack.push(StackElem {
                            pos: to_visit,
                            so_far: so_far + 1,
                            data: StackElemData::Encountered,
                        });
                    } else {
                        // no more elements to traverse.
                        stack.push(StackElem {
                            pos,
                            so_far,
                            data: StackElemData::Finalized { min_path },
                        });
                    }
                }
            }
            // for possible_move in
            // {
            //     to_visit.push((possible_move, so_far + 1));
            // }
        }
        None
    }
}

struct StackElem {
    pos: Coord,
    so_far: usize,
    data: StackElemData,
}

#[derive(Debug)]
enum StackElemData {
    Encountered,
    Visiting { i: usize, min_path: Option<usize> },
    Finalized { min_path: Option<usize> },
}
