use nom::{
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let v: IResult<&str, Vec<Vec<usize>>> = separated_list1(
        newline,
        separated_list1(space1, map_res(digit1, |d: &str| d.parse::<usize>())),
    )(input);
    v.unwrap().1
}

#[derive(PartialEq, Debug)]
enum Direction {
    Increasing,
    Decreasing,
    None,
}

pub fn is_safe(report: Vec<usize>) -> bool {
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.

    let mut direction = Direction::None;
    report.windows(2).all(|w| {
        let diff = w[1] as i32 - w[0] as i32;
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        let new_direction = if diff > 0 {
            Direction::Increasing
        } else {
            Direction::Decreasing
        };
        if direction != Direction::None && direction != new_direction {
            return false;
        }
        direction = new_direction;
        true
    })
}
