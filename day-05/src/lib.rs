use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub type PageRule = (usize, usize);
pub type PageRules = Vec<PageRule>;
pub type PrintSequence = Vec<usize>;

trait Parse {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parse for PageRule {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, pagerule) = separated_pair(
            map_res(digit1, |d: &str| d.parse::<usize>()),
            tag("|"),
            map_res(digit1, |d: &str| d.parse::<usize>()),
        )(input)?;
        Ok((input, pagerule))
    }
}

impl Parse for PageRules {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(newline, PageRule::parse)(input)
    }
}

impl Parse for PrintSequence {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(tag(","), map_res(digit1, |d: &str| d.parse::<usize>()))(input)
    }
}

pub fn parse_input(input: &str) -> (PageRules, Vec<PrintSequence>) {
    separated_pair(
        PageRules::parse,
        tag("\n\n"),
        separated_list1(newline, PrintSequence::parse),
    )(input)
    .unwrap()
    .1
}

pub fn page_rules_map(page_rules: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut map = HashMap::new();
    page_rules.iter().for_each(|(a, b)| {
        map.entry(*a)
            .and_modify(|v: &mut Vec<usize>| v.push(*b))
            .or_insert(vec![*b]);
    });
    map
}

pub fn is_correctly_ordered(
    print_sequence: &PrintSequence,
    page_rules_map: &HashMap<usize, Vec<usize>>,
) -> bool {
    print_sequence
        .iter()
        .enumerate()
        .all(|(print_order, page)| {
            // if a rule exists for the current page
            if let Some(rule) = page_rules_map.get(page) {
                // loop through all of the pages that must precede the current page
                // check if it is in the current print sequence
                rule.iter().all(|must_precede_page| {
                    // if it is in the print sequence, check that it is after the current page
                    if let Some(index) = print_sequence.iter().position(|p| p == must_precede_page)
                    {
                        index > print_order
                    } else {
                        // if it is not in the current print sequence, then no rules have been broken
                        true
                    }
                })
            } else {
                // if no rules exist for the current page, then no rules have been broken
                true
            }
        })
}
