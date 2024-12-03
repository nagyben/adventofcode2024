use regex::Regex;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    parsed.iter().fold(0, |acc, (a, b)| acc + a * b)
}

pub fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut mults = vec![];
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        mults.push((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
    }
    mults
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str =
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(process(INPUT), 161);
    }
}
