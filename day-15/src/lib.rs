use color_eyre::eyre::eyre;
use nom::{
    character::complete::{anychar, multispace0, newline},
    combinator::map_res,
    error::Error,
    multi::{many1, separated_list1},
    IResult,
};

pub type Grid = Vec<Vec<ObstacleType>>;

#[derive(Debug, Copy, Clone)]
pub enum ObstacleType {
    Wall,
    Box,
    Empty,
    Robot,
}

impl std::fmt::Display for ObstacleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObstacleType::Wall => write!(f, "#"),
            ObstacleType::Box => write!(f, "O"),
            _ => write!(f, "."),
        }
    }
}

pub fn parse_grid(input: &str) -> IResult<&str, Grid> {
    separated_list1(
        newline,
        many1(map_res(anychar, |c| match c {
            '.' => Ok(ObstacleType::Empty),
            'O' => Ok(ObstacleType::Box),
            '#' => Ok(ObstacleType::Wall),
            '@' => Ok(ObstacleType::Robot),
            _ => Err(eyre!("Invalid character in input: {}", c)),
        })),
    )(input)
}

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<glam::IVec2>> {
    map_res(
        separated_list1(
            newline,
            many1(map_res(anychar, |c: char| match c {
                '^' => Ok(glam::IVec2 { x: 0, y: -1 }),
                '>' => Ok(glam::IVec2 { x: 1, y: 0 }),
                'v' => Ok(glam::IVec2 { x: 0, y: 1 }),
                '<' => Ok(glam::IVec2 { x: -1, y: 0 }),
                _ => Err(eyre!("Invalid character in input: {}", c)),
            })),
        ),
        |list| Ok::<std::vec::Vec<glam::IVec2>, Error<&str>>(list.into_iter().flatten().collect()),
    )(input)
}

pub fn parse_input(input: &str) -> (Grid, Vec<glam::IVec2>, glam::IVec2) {
    let (input, mut grid) = parse_grid(input).unwrap();
    let (input, _) = multispace0::<&str, Error<&str>>(input).unwrap();
    let (_, instructions) = parse_instructions(input).unwrap();
    let start_position = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|c| matches!(c, ObstacleType::Robot))
                .map(|x| glam::IVec2 {
                    x: x as i32,
                    y: y as i32,
                })
        })
        .unwrap();
    (grid, instructions, start_position)
}
