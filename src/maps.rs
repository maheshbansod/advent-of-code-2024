#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
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

    pub fn at_checked(&self, p: Coord) -> Option<&T> {
        self.data.get(p.0).and_then(|row| row.get(p.1))
    }
}
impl<T: Copy + PartialEq> Grid<T> {
    pub fn find_position(&self, needle: T) -> Option<Coord> {
        self.data.iter().enumerate().find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, c)| (*c == needle).then_some((i, j)))
        })
    }
}
