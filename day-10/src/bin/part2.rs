use itertools::Itertools;
use pathfinding::prelude::count_paths;

use day_10::parse_input;

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let map = parse_input(input);
    let starts: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &height)| height == 0)
                .map(move |(x, _)| (y, x))
        })
        .collect();
    let ends: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &height)| height == 9)
                .map(move |(x, _)| (y, x))
        })
        .collect();

    // create a cartesian product of all possible start and end positions
    // and calculate the trailhead rating for each pair
    // if a pair of start and end positions is not connected, the rating will be 0
    ends.iter()
        .cartesian_product(starts.iter())
        .map(|(end, start)| trailhead_rating(*start, *end, &map))
        .sum()
}

fn trailhead_rating(
    start_position: (usize, usize),
    end_position: (usize, usize),
    map: &Vec<Vec<usize>>,
) -> usize {
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    // the pathfinding crate is OP
    count_paths(
        start_position,
        |&(y, x)| {
            let height = map[y][x];
            let mut neighbors = vec![];
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
                if map[new_position.0][new_position.1] == height + 1 {
                    neighbors.push((new_position.0, new_position.1));
                }
            }
            neighbors
        },
        |&position| position == end_position,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(process(INPUT), 81);
    }
}
