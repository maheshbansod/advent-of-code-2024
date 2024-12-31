use std::fmt::Display;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn move_pos_checked(&self, next_pos: Coord) -> Option<Coord> {
        let (i, j) = next_pos;
        match self {
            Direction::Up => {
                if i > 0 {
                    Some((i - 1, j))
                } else {
                    None
                }
            }
            Direction::Down => Some((i + 1, j)),
            Direction::Left => {
                if j > 0 {
                    Some((i, j - 1))
                } else {
                    None
                }
            }
            Direction::Right => Some((i, j + 1)),
        }
    }

    pub const fn move_pos(&self, next_pos: Coord) -> Coord {
        let (i, j) = next_pos;
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }

    pub const fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    pub fn diff(&self, other: &Direction) -> u8 {
        match (self, other) {
            (Direction::Up, Direction::Down) => 2,
            (Direction::Down, Direction::Up) => 2,
            (Direction::Left, Direction::Right) => 2,
            (Direction::Right, Direction::Left) => 2,
            (Direction::Up | Direction::Down, Direction::Left | Direction::Right) => 1,
            (Direction::Left | Direction::Right, Direction::Up | Direction::Down) => 1,
            (a, b) if a == b => 0,
            _ => panic!(),
        }
    }
}

pub type Coord = (usize, usize);

#[derive(Clone)]
pub struct Grid<T: Copy> {
    data: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub const fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn at(&self, p: Coord) -> T {
        self.data[p.0][p.1]
    }

    pub fn at_mut(&mut self, p: Coord) -> &mut T {
        &mut self.data[p.0][p.1]
    }

    pub fn at_checked(&self, p: Coord) -> Option<&T> {
        self.data.get(p.0).and_then(|row| row.get(p.1))
    }

    pub fn is_edge(&self, p: &Coord) -> bool {
        p.0 == 0
            || p.0 == self.data.len() - 1
            || p.1 == 0
            || self.data.first().is_none_or(|row| row.len() - 1 == p.1)
    }
}
impl<T: Copy + Display> Grid<T> {
    pub fn pretty_print(&self) {
        for row in self.data.iter() {
            for c in row {
                print!("{c}");
            }
            println!();
        }
    }
}
impl<'a, T: Copy + PartialEq> Grid<T> {
    pub fn find_position(&self, needle: T) -> Option<Coord> {
        self.data.iter().enumerate().find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, c)| (*c == needle).then_some((i, j)))
        })
    }
    pub fn find_all_positions(&'a self, needle: T) -> impl Iterator<Item = Coord> + 'a {
        self.data.iter().enumerate().flat_map(move |(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| (*c == needle).then_some((i.clone(), j)))
        })
    }
}

pub fn is_out_of_bounds(pos: Coord, ilim: usize, jlim: usize) -> bool {
    pos.0 >= ilim || pos.1 >= jlim
}
