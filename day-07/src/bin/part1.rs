use day_07::{parse_input, Operator};

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    println!("{:?}", parsed);
    parsed
        .iter()
        .filter(|(target_sum, operands)| possible_to_calculate(*target_sum, operands))
        .map(|(target_sum, _)| *target_sum)
        .sum()
}

fn possible_to_calculate(target_sum: usize, operands: &[usize]) -> bool {
    let mut operators = vec![Operator::Add; operands.len() - 1];

    false
}

fn calculate(operands: &[usize], operators: &[Operator]) -> usize {
    assert!(operands.len() == operators.len() + 1);
    let mut result = operands[0];
    for (op, operand) in operators.iter().zip(operands.iter().skip(1)) {
        match op {
            Operator::Add => result += operand,
            Operator::Multiply => result *= operand,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn example() {
        const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        assert_eq!(process(INPUT), 3749);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(
            calculate(&[1, 2, 3], &[Operator::Add, Operator::Multiply]),
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
    fn test_possible_to_calculate(
        #[case] target_sum: usize,
        #[case] operands: &[usize],
        #[case] expected: bool,
    ) {
        assert_eq!(possible_to_calculate(target_sum, operands), expected);
    }
}
