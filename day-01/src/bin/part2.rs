use day01::parse_input;

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut parsed = parse_input(input);
    parsed.1.sort();
    parsed.0.iter().fold(0, |acc, a| {
        acc + a * parsed.1.iter().filter(|b| a == *b).count()
    })
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
        assert_eq!(process(INPUT), 31);
    }
}
