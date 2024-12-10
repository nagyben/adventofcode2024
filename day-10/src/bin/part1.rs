use std::collections::{HashSet, VecDeque};

use day_10::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    let starts: Vec<(usize, usize)> = parsed
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &height)| height == 0)
                .map(move |(x, _)| (y, x))
        })
        .collect();

    starts
        .iter()
        .map(|&start| trailhead_score(start, &parsed))
        .sum()
}

fn trailhead_score(start_position: (usize, usize), map: &Vec<Vec<usize>>) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::from(vec![start_position]);
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut score = 0;

    while !to_visit.is_empty() {
        let position = to_visit.pop_front().unwrap();
        visited.insert(position);
        let (y, x) = position;
        let height = map[y][x];
        if height == 9 {
            score += 1;
        }
        for (dy, dx) in DIRECTIONS.iter() {
            let new_position = (y as isize + dy, x as isize + dx);
            if new_position.0 < 0
                || new_position.1 < 0
                || new_position.0 >= map.len() as isize
                || new_position.1 >= map[0].len() as isize
            {
                continue;
            }
            let new_position = (new_position.0 as usize, new_position.1 as usize);
            if visited.contains(&new_position) {
                continue;
            }
            if to_visit.contains(&new_position) {
                continue;
            }
            if map[new_position.0][new_position.1] == height + 1 {
                to_visit.push_back(new_position);
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn example() {
        const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!(process(INPUT), 36);
    }

    #[rstest]
    #[case((0,2), 5)]
    #[case((0,4), 6)]
    fn test_trailhead_score(#[case] start_position: (usize, usize), #[case] expected: usize) {
        const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        let map = parse_input(INPUT);
        assert_eq!(trailhead_score(start_position, &map), expected);
    }
}
