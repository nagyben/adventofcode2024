use nom::{
    character::complete::{digit1, newline, space1}, combinator::map_res, multi::separated_list1, sequence::separated_pair, Err, IResult
};

type Grid = Vec<Vec<Obstacle>>;

struct Obstacle {
    position: glam::IVec2,
    obstacle_type: ObstacleType,
}

impl Obstacle {
    pub fn try_move(&self, &mut grid: Grid) -> Result<()>> {
        todo!()
    }
}

enum ObstacleType {
    Wall,
    Box,
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let v: IResult<&str, Vec<(usize, usize)>> = separated_list1(
        newline,
        separated_pair(
            map_res(digit1, |d: &str| d.parse::<usize>()),
            space1,
            map_res(digit1, |d: &str| d.parse::<usize>()),
        ),
    )(input);
    v.unwrap().1.iter().cloned().unzip()
}
