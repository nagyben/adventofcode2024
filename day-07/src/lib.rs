use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    let v: IResult<&str, Vec<(usize, Vec<usize>)>> = separated_list1(
        newline,
        separated_pair(
            map_res(digit1, |d: &str| d.parse::<usize>()),
            tag(": "),
            separated_list1(space1, map_res(digit1, |d: &str| d.parse::<usize>())),
        ),
    )(input);
    v.unwrap().1
}

pub fn possible_to_calculate(
    target_sum: usize,
    operands: &[usize],
    operators: &Vec<Operator>,
) -> bool {
    repeat_n(operators.clone(), operands.len() - 1)
        .multi_cartesian_product()
        .any(|ops| calculate(operands, &ops) == target_sum)
}

fn calculate(operands: &[usize], operators: &Vec<Operator>) -> usize {
    assert!(operands.len() == operators.len() + 1);
    let mut result = operands[0];
    for (op, operand) in operators.iter().zip(operands.iter().skip(1)) {
        match op {
            Operator::Add => result += operand,
            Operator::Multiply => result *= operand,
            Operator::Concatenate => {
                result = result * (usize::pow(10, operand.ilog10() + 1)) + operand
            }
        }
    }
    result
}

#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_calculate() {
        assert_eq!(
            calculate(&[1, 2, 3], &vec![Operator::Add, Operator::Multiply]),
            9
        );
    }

    #[rstest]
    #[case(190, &[10, 19], true)]
    #[case(3267, &[81, 40, 27], true)]
    #[case(83, &[17, 5], false)]
    #[case(156, &[15, 6], false)]
    #[case(7290, &[6, 8, 6, 15], false)]
    #[case(161011, &[16, 10, 13], false)]
    #[case(192, &[17, 8, 14], false)]
    #[case(21037, &[9, 7, 18, 13], false)]
    #[case(292, &[11, 6, 16, 20], true)]
    fn test_possible_to_calculate_add_multiply(
        #[case] target_sum: usize,
        #[case] operands: &[usize],
        #[case] expected: bool,
    ) {
        assert_eq!(
            possible_to_calculate(
                target_sum,
                operands,
                &vec![Operator::Add, Operator::Multiply]
            ),
            expected
        );
    }

    #[rstest]
    #[case(190, &[10, 19], true)]
    #[case(3267, &[81, 40, 27], true)]
    #[case(83, &[17, 5], false)]
    #[case(156, &[15, 6], true)] // now true
    #[case(7290, &[6, 8, 6, 15], true)] // now true
    #[case(161011, &[16, 10, 13], false)]
    #[case(192, &[17, 8, 14], true)] // now true
    #[case(21037, &[9, 7, 18, 13], false)]
    #[case(292, &[11, 6, 16, 20], true)]
    fn test_possible_to_calculate_add_multiply_concatenate(
        #[case] target_sum: usize,
        #[case] operands: &[usize],
        #[case] expected: bool,
    ) {
        assert_eq!(
            possible_to_calculate(
                target_sum,
                operands,
                &vec![Operator::Add, Operator::Multiply, Operator::Concatenate]
            ),
            expected
        );
    }
}
