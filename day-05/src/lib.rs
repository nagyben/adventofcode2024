use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub type PageRule = (usize, usize);
type PageRules = Vec<PageRule>;
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
