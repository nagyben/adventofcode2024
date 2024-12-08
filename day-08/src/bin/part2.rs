use itertools::Itertools;
use std::collections::HashMap;

use day_08::{get_antenna_coords, parse_input};

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    let antenna_coords = get_antenna_coords(&parsed);
    let antinodes = get_antinodes_with_harmonics(&antenna_coords, &parsed);
    antinodes.iter().flatten().filter(|&&x| x).count()
}

fn get_antinodes_with_harmonics(
    antenna_coords: &HashMap<char, Vec<(usize, usize)>>,
    parsed: &Vec<Vec<char>>,
) -> Vec<Vec<bool>> {
    let mut antinodes: Vec<Vec<bool>> = vec![vec![false; parsed[0].len()]; parsed.len()];
    antenna_coords.iter().for_each(|(frequency, coords)| {
        coords
            .iter()
            .tuple_combinations()
            .for_each(|(coord1, coord2)| {
                let (dy, dx) = (
                    coord2.0 as isize - coord1.0 as isize,
                    coord2.1 as isize - coord1.1 as isize,
                );
                let antinodes_1 =
                    (0..50).map(|i| (coord2.0 as isize + i * dy, coord2.1 as isize + i * dx));
                let antinodes_2 =
                    (0..50).map(|i| (coord1.0 as isize - i * dy, coord1.1 as isize - i * dx));

                for antinode_1 in antinodes_1 {
                    if antinode_1.0 >= 0
                        && antinode_1.0 < parsed.len() as isize
                        && antinode_1.1 >= 0
                        && antinode_1.1 < parsed[0].len() as isize
                    {
                        antinodes[antinode_1.0 as usize][antinode_1.1 as usize] = true;
                    }
                }

                for antinode_2 in antinodes_2 {
                    if antinode_2.0 >= 0
                        && antinode_2.0 < parsed.len() as isize
                        && antinode_2.1 >= 0
                        && antinode_2.1 < parsed[0].len() as isize
                    {
                        antinodes[antinode_2.0 as usize][antinode_2.1 as usize] = true;
                    }
                }
            })
    });
    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
"#;
        assert_eq!(process(INPUT), 9);
    }

    #[test]
    fn example_2() {
        const INPUT: &str = r#"##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##
"#;
        assert_eq!(process(INPUT), 34);
    }
}
