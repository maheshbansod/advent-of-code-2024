use std::{collections::HashSet, fs};

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(6, false);
    let data = fs::read_to_string(file_path)?;
    let data: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut guard = Guard::from_field(data.clone());
    let first_place = guard.position;
    let mut visited_places = Vec::new();
    let (_, visited_places) = guard.traverse_field(&mut visited_places);
    let mut unique_places: Vec<(Coord, Direction)> = Vec::new();
    for p in visited_places.iter() {
        if !unique_places.iter().any(|u| u.0 == p.0) {
            unique_places.push(*p);
        }
    }
    println!("Visited {} places.", unique_places.len());

    // let mut known_loops: Vec<Vec<(Coord, Direction)>> = vec![];
    let mut count = 0;
    for (_i, (pos, visited_direction)) in unique_places.iter().enumerate().skip(1) {
        if *pos == first_place {
            continue;
        }
        let mut data = data.clone();
        data[pos.y][pos.x] = '#';
        let mut guard = Guard::from_field(data);
        let before_pos = Guard::next_move_from_direction(
            &visited_direction.opposite(),
            &(pos.x as i32, pos.y as i32),
        );
        guard.position = Coord::from(&before_pos);
        guard.facing = visited_direction.turn_right();
        // let available_loops = known_loops
        //     .iter()
        //     .filter(|l| l.iter().any(|(kp, kd)| kp == pos))
        //     .collect::<Vec<_>>();
        let has_loop = guard.check_loop();
        if has_loop {
            count += 1;
            // known_loops.push(new_visited_places);
            // println!("{i} covered: {count} loops");
        }
    }

    println!("count: {count}");

    Ok(())
}

type Data = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    const fn from(pair: &(i32, i32)) -> Self {
        Coord {
            x: pair.0 as usize,
            y: pair.1 as usize,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    const fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
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

    fn check_loop(&mut self) -> bool {
        let mut visited_places = HashSet::<(Coord, Direction)>::new();
        visited_places.insert((self.position, self.facing));
        loop {
            let forward_status = self.move_forward();
            for place in forward_status.covered {
                if visited_places.contains(&(place, self.facing)) {
                    return true;
                }
                visited_places.insert((place, self.facing));
            }
            match forward_status.stop_reason {
                StopReason::Outside => return false,
                StopReason::Obstacle => {
                    self.turn_right();
                }
            }
        }
    }

    fn traverse_field<'a>(
        &mut self,
        visited_places: &'a mut Vec<(Coord, Direction)>,
    ) -> (bool, &'a Vec<(Coord, Direction)>) {
        visited_places.clear();
        visited_places.push((self.position, self.facing));
        loop {
            let forward_status = self.move_forward();
            for place in forward_status.covered {
                visited_places.push((place, self.facing));
            }
            match forward_status.stop_reason {
                StopReason::Outside => return (false, visited_places),
                StopReason::Obstacle => {
                    self.turn_right();
                }
            }
        }
    }

    fn _print_surrounding(&self) {
        let curr_elem = self._elem_at(&self.position);
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
            let next_pos = Guard::next_move_from_direction(
                &direction,
                &(self.position.x as i32, self.position.y as i32),
            );
            let elem = self._elem_at(&Coord::from(&next_pos));
            println!("{:?}: {:?} {:?}", direction, next_pos, elem);
        }
    }

    fn _elem_at(&self, position: &Coord) -> Option<&char> {
        self.field.get(position.y)?.get(position.x)
    }

    fn move_forward(&mut self) -> ForwardStatus {
        let mut covered = vec![];
        let next_pos = self.next_step();
        let diff = (
            next_pos.0 as i32 - self.position.x as i32,
            next_pos.1 - self.position.y as i32,
        );
        loop {
            let next_pos = (
                self.position.x as i32 + diff.0,
                self.position.y as i32 + diff.1,
            );
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

    const fn next_step(&self) -> (i32, i32) {
        Guard::next_move_from_direction(
            &self.facing,
            &(self.position.x as i32, self.position.y as i32),
        )
    }

    const fn next_move_from_direction(direction: &Direction, pair: &(i32, i32)) -> (i32, i32) {
        match direction {
            Direction::Up => (pair.0, pair.1 - 1),
            Direction::Down => (pair.0, pair.1 + 1),
            Direction::Left => (pair.0 - 1, pair.1),
            Direction::Right => (pair.0 + 1, pair.1),
        }
    }

    const fn turn_right(&mut self) {
        self.facing = self.facing.turn_right();
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
