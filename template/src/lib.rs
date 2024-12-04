use nom::{
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

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
