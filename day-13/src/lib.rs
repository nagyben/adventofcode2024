use std::cmp::max;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub struct ButtonRule {
    pub dx: isize,
    pub dy: isize,
}

impl ButtonRule {
    /// parses one of the following:
    /// Button A: X+94, Y+34
    /// Button B: X+22, Y+67
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = preceded(tag("Button "), alpha1)(input)?;
        let (input, dx) =
            preceded(tag(": X+"), map_res(digit1, |d: &str| d.parse::<isize>()))(input)?;
        let (input, _) = tag(", Y+")(input)?;
        let (input, dy) = map_res(digit1, |d: &str| d.parse::<isize>())(input)?;
        Ok((input, Self { dx, dy }))
    }
}

#[derive(PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, x) = preceded(
            tag("Prize: X="),
            map_res(digit1, |d: &str| d.parse::<isize>()),
        )(input)?;
        let (input, y) =
            preceded(tag(", Y="), map_res(digit1, |d: &str| d.parse::<isize>()))(input)?;
        Ok((input, Self { x, y }))
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Machine {
    pub prize: Point,
    pub button_a: ButtonRule,
    pub button_b: ButtonRule,
}

impl Machine {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = multispace0(input)?;
        let (input, (button_a, button_b)) =
            separated_pair(ButtonRule::parse, newline, ButtonRule::parse)(input)?;
        let (input, _) = multispace0(input)?;
        let (input, prize) = Point::parse(input)?;
        Ok((
            input,
            Self {
                prize,
                button_a,
                button_b,
            },
        ))
    }
}

pub fn parse_input(input: &str) -> Vec<Machine> {
    let (_, machines) = separated_list1(tag("\n\n"), Machine::parse)(input).unwrap();
    machines
}

