use crate::{parse_input, run_program};

pub fn process(input: &str) -> String {
    let program = parse_input(input);
    run_program(&program)
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        assert_eq!(process(INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example_2() {
        const INPUT: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        assert_eq!(process(INPUT), "4,2,5,6,7,7,7,7,3,1,0");
    }

    #[test]
    fn example_3() {
        const INPUT: &str = r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"#;
        assert_eq!(process(INPUT), "0,1,2");
    }
}
