use crate::{generate_n_secrets, parse_input};

pub fn process(input: &str) -> usize {
    let secrets = parse_input(input);
    secrets
        .iter()
        .map(|secret| *generate_n_secrets(*secret, 2000).last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"1
10
100
2024"#;
        assert_eq!(process(INPUT), 37327623);
    }
}
