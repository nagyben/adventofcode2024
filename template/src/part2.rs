use crate::parse_input;

pub fn process(input: &str) -> usize {
    let parsed = parse_input(input);
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
