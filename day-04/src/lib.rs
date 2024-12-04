use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    let v: IResult<&str, Vec<Vec<char>>> = separated_list1(newline, many1(one_of("XMAS")))(input);
    v.unwrap().1
}
