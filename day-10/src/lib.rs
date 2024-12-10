pub fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| char::to_digit(c, 10).unwrap() as usize)
                .collect()
        })
        .collect()
}
