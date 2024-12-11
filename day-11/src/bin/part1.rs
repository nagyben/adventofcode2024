use std::collections::LinkedList;

use day_11::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut stones = parse_input(input);
    blink(&mut stones);
    0
}

fn blink(stones: &mut LinkedList<usize>) {
    let mut iter = stones.iter_mut().enumerate();
    while let Some((i, value)) = iter.next() {
        if *value == 0 {
            *value = 1;
            continue;
        }
        let num_digits = value.ilog10() + 1;
        if num_digits % 2 == 0 {
            let left = *value / 10usize.pow(num_digits / 2);
            let right = *value % 10usize.pow(num_digits / 2);
            let mut tail = stones.split_off(i + 1);
            stones.push_back(left);
            stones.push_back(right);
            stones.append(&mut tail);
        }
    }
    todo!()
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
