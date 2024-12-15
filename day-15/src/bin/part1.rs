use day_15::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut parsed = parse_input(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#""#;
        assert_eq!(process(INPUT), todo!());
    }
}
