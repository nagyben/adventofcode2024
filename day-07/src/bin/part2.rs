use day_07::{parse_input, possible_to_calculate, Operator};

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    let operators = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];
    parsed
        .iter()
        .filter(|(target_sum, operands)| possible_to_calculate(*target_sum, operands, &operators))
        .map(|(target_sum, _)| *target_sum)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(process(INPUT), 11387);
    }
}
