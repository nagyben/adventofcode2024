use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, newline, space1},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse::<isize>()
    })(input)?;

    Ok((i, number))
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, position) = preceded(
        tag("p="),
        separated_pair(parse_isize, tag(","), parse_isize),
    )(input)?;
    let (input, velocity) = preceded(
        preceded(multispace0, tag("v=")),
        separated_pair(parse_isize, tag(","), parse_isize),
    )(input)?;
    Ok((
        input,
        Robot {
            position: glam::IVec2 {
                x: position.0 as i32,
                y: position.1 as i32,
            },
            velocity: glam::IVec2 {
                x: velocity.0 as i32,
                y: velocity.1 as i32,
            },
        },
    ))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(newline, parse_robot)(input)
}

#[derive(Debug)]
pub struct Robot {
    pub position: IVec2,
    pub velocity: IVec2,
}

impl Robot {
    pub fn tick(&mut self, max_x: i32, max_y: i32) {
        self.position += self.velocity;
        if self.position.x > max_x {
            self.position.x -= max_x + 1
        }
        if self.position.x < 0 {
            self.position.x += max_x + 1
        }

        if self.position.y > max_y {
            self.position.y -= max_y + 1
        }

        if self.position.y < 0 {
            self.position.y += max_y + 1
        }
    }
}

pub type Robots = Vec<Robot>;
