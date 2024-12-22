use day_17::part2::process;

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
        assert_eq!(process(INPUT), 117440);
    }
}
