use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map_res, value},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut state = true;
    let instructions = parse_input(input);
    instructions
        .iter()
        .fold(0, |acc, instruction| match instruction {
            Instruction::Do => {
                state = true;
                acc
            }
            Instruction::Dont => {
                state = false;
                acc
            }
            Instruction::Mul(a, b) => {
                if state {
                    acc + (*a as usize) * (*b as usize)
                } else {
                    acc
                }
            }
        })
}

fn integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[derive(Debug, Clone)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(integer, tag(","), integer),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Do, tag("do()")),
        value(Instruction::Dont, tag("don't()")),
        mul,
    ))(input)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    many1(many_till(anychar, instruction).map(|(_, instruction)| instruction))(input)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str =
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(process(INPUT), 48);
    }
}
