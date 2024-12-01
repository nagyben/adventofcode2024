use day01::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut parsed = parse_input(input);
    parsed.0.sort();
    parsed.1.sort();
    parsed
        .0
        .iter()
        .zip(parsed.1.iter())
        .fold(0, |acc, (a, b)| acc + (*a).abs_diff(*b))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        assert_eq!(process(INPUT), 11);
    }
}
