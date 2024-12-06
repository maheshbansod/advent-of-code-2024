use std::{collections::HashSet, fs};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(6, false);
    let data = fs::read_to_string(file_path)?;
    let data: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut guard = Guard::from_field(data);
    let visited_places = guard.move_guard();
    println!("visited: {}", visited_places);
    Ok(())
}

type Data = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn from(pair: &(i32, i32)) -> Self {
        Coord {
            x: pair.0 as usize,
            y: pair.1 as usize,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const GUARD_FACING: [(char, Direction); 4] = [
    ('^', Direction::Up),
    ('v', Direction::Down),
    ('>', Direction::Right),
    ('<', Direction::Left),
];

struct Guard {
    position: Coord,
    facing: Direction,
    field: Data,
}

impl Guard {
    fn from_field(data: Data) -> Guard {
        for (i, line) in data.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if let Some((position, facing)) = Guard::get_guard_info(c, j, i) {
                    return Self {
                        field: data,
                        facing,
                        position,
                    };
                }
            }
        }
        panic!("No guard - that's unexpected");
    }
    fn get_guard_info(c: &char, x: usize, y: usize) -> Option<(Coord, Direction)> {
        let (_, direction) = GUARD_FACING.iter().find(|(g, _)| g == c)?;
        Some((Coord { x, y }, *direction))
    }

    fn move_guard(&mut self) -> usize {
        let mut visited_places = HashSet::new();
        visited_places.insert(self.position);
        loop {
            // println!("starting from {:?}, {:?}", self.position, self.facing);
            let forward_status = self.move_forward();
            // println!("covered: {}", forward_status.covered.len());
            // println!("moved to {:?}", self.position);
            for place in forward_status.covered {
                visited_places.insert(place);
            }
            match forward_status.stop_reason {
                StopReason::Outside => return visited_places.len(),
                StopReason::Obstacle => {
                    self.print_surrounding();
                    self.turn_right();
                }
            }
        }
    }

    fn print_surrounding(&self) {
        let curr_elem = self.elem_at(&self.position);
        println!(
            "current position: {:?}: {:?} {:?}",
            self.position, curr_elem, self.facing
        );
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let next_pos = Guard::move_coord(
                &direction,
                &(self.position.x as i32, self.position.y as i32),
            );
            let elem = self.elem_at(&Coord::from(&next_pos));
            println!("{:?}: {:?} {:?}", direction, next_pos, elem);
        }
    }

    fn elem_at(&self, position: &Coord) -> Option<&char> {
        self.field.get(position.y)?.get(position.x)
    }

    fn move_forward(&mut self) -> ForwardStatus {
        let mut covered = vec![];
        loop {
            let next_pos = self.next_step();
            let next_pos_thing = (next_pos.0 >= 0 && next_pos.1 >= 0)
                .then(|| {
                    self.field
                        .get(next_pos.1 as usize)
                        .map(|row| row.get(next_pos.0 as usize))
                })
                .flatten()
                .flatten();
            if let Some(next_pos_thing) = next_pos_thing {
                if *next_pos_thing != '#' {
                    let next_pos = Coord::from(&next_pos);
                    covered.push(next_pos);
                    self.position = next_pos;
                } else {
                    break;
                }
            } else {
                return ForwardStatus {
                    stop_reason: StopReason::Outside,
                    covered,
                };
            }
        }
        ForwardStatus {
            stop_reason: StopReason::Obstacle,
            covered,
        }
    }

    fn next_step(&self) -> (i32, i32) {
        Guard::move_coord(
            &self.facing,
            &(self.position.x as i32, self.position.y as i32),
        )
    }

    const fn move_coord(direction: &Direction, pair: &(i32, i32)) -> (i32, i32) {
        match direction {
            Direction::Up => (pair.0, pair.1 - 1),
            Direction::Down => (pair.0, pair.1 + 1),
            Direction::Left => (pair.0 - 1, pair.1),
            Direction::Right => (pair.0 + 1, pair.1),
        }
    }

    fn turn_right(&mut self) {
        match self.facing {
            Direction::Up => {
                self.facing = Direction::Right;
            }
            Direction::Down => {
                self.facing = Direction::Left;
            }
            Direction::Left => {
                self.facing = Direction::Up;
            }
            Direction::Right => {
                self.facing = Direction::Down;
            }
        }
    }
}

struct ForwardStatus {
    stop_reason: StopReason,
    covered: Vec<Coord>,
}

enum StopReason {
    Outside,
    Obstacle,
}
