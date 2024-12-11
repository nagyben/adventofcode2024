use std::collections::LinkedList;

use day_11::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut stones = parse_input(input);
    for _ in 0..25 {
        stones = blink(stones);
    }
    stones.len()
}

fn blink(mut stones: LinkedList<usize>) -> LinkedList<usize> {
    let mut new_stones = LinkedList::new();
    while let Some(n) = stones.pop_front() {
        if n == 0 {
            new_stones.push_back(1);
            continue;
        }
        let num_digits = n.ilog10() + 1;
        if num_digits % 2 == 0 {
            let left = n / 10usize.pow(num_digits / 2);
            let right = n % 10usize.pow(num_digits / 2);
            new_stones.push_back(left);
            new_stones.push_back(right);
        } else {
            new_stones.push_back(n * 2024);
        }
    }
    new_stones
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"125 17"#;
        assert_eq!(process(INPUT), 55312);
    }
}
