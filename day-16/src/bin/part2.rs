use std::collections::HashSet;

use day_16::{parse_input, Tile};
use pathfinding::prelude::{astar_bag, dijkstra, yen};

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let (maze, start_pos, end_pos) = parse_input(input);
    let paths = astar_bag(
        &(start_pos, glam::IVec2::X),
        |(position, direction)| {
            let next_pos = position + direction;
            if let Tile::Wall = maze[next_pos.y as usize][next_pos.x as usize] {
                vec![
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            } else {
                vec![
                    ((next_pos, *direction), 1),
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            }
        },
        |(_, _)| 1000000,
        |(position, _)| *position == end_pos,
    )
    .expect("a path solution wasn't found")
    .0;
    paths
        .flat_map(|path| path.into_iter().map(|(pos, _)| pos))
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        assert_eq!(process(INPUT), 45);
    }

    #[test]
    fn example_2() {
        const INPUT: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        assert_eq!(process(INPUT), 64);
    }
}
