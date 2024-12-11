use std::collections::LinkedList;

use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn parse_input(input: &str) -> LinkedList<usize> {
    let mut list = LinkedList::new();
    let v: IResult<&str, Vec<usize>> =
        separated_list1(space1, map_res(digit1, |d: &str| d.parse::<usize>()))(input);
    v.unwrap().1.iter().for_each(|&x| list.push_back(x));
    list
}
