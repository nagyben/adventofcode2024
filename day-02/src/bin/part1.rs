use day02::is_safe;
use day02::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    parsed
        .iter()
        .map(|report| is_safe(report.clone()) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![7,6,4,2,1], true)]
    #[case(vec![1,2,7,8,9], false)]
    #[case(vec![9,7,6,2,1], false)]
    #[case(vec![1,3,2,4,5], false)]
    #[case(vec![8,6,4,4,1], false)]
    #[case(vec![1,3,6,7,9], true)]
    fn test_is_safe(#[case] input: Vec<usize>, #[case] expected: bool) {
        assert_eq!(expected, is_safe(input))
    }

    #[test]
    fn example() {
        const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(process(INPUT), 2);
    }
}
