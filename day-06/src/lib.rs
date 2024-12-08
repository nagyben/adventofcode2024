use std::fmt::{self, Debug, Formatter};

pub type Grid = Vec<Vec<char>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Debug for GuardDirection {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            GuardDirection::Up => write!(f, "^"),
            GuardDirection::Down => write!(f, "v"),
            GuardDirection::Left => write!(f, "<"),
            GuardDirection::Right => write!(f, ">"),
        }
    }
}

pub type StartPosition = (usize, usize);

pub fn parse_input(input: &str) -> (Grid, StartPosition, GuardDirection) {
    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
    let mut direction = GuardDirection::Up;
    let start_position = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&ch| {
                    if ch == '^' {
                        direction = GuardDirection::Up;
                        true
                    } else if ch == 'v' {
                        direction = GuardDirection::Down;
                        true
                    } else if ch == '<' {
                        direction = GuardDirection::Left;
                        true
                    } else if ch == '>' {
                        direction = GuardDirection::Right;
                        true
                    } else {
                        false
                    }
                })
                .map(|j| (j, i))
        })
        .unwrap();
    (grid, start_position, direction)
}
