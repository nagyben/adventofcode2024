use day_16::parse_input;
use petgraph::graph::UnGraph;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let (start_pos, maze) = parse_input(input);
    let mut maze_graph: UnGraph<(usize, usize), usize> = UnGraph::default();
    let directions = [glam::IVec2::new(0, 1), glam::IVec2::new(1, 0), glam::IVec2::new(0, -1), glam::IVec2::new(-1, 0)];
    maze.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, tile)| {
            if *tile == Tile::Wall {
                return;
            }
            let current_node = maze_graph.add_node((x, y));
            for direction in directions.iter() {
                let new_pos = glam::IVec2::new(x as i32, y as i32) + *direction;
                if new_pos.x < 0 || new_pos.y < 0 {
                    continue;
                }
                if let Some(new_tile) = maze.get(new_pos.y as usize).and_then(|row| row.get(new_pos.x as usize)) {
                    if *new_tile == Tile::Wall {
                        continue;
                    }
                    let new_node = maze_graph.add_node((new_pos.x as usize, new_pos.y as usize));
                    maze_graph.add_edge(current_node, new_node, 1);
                }
            }
        });
    });

        maze_graph.add_node(tile);
    todo!()
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
        assert_eq!(process(INPUT), 7036);
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
        assert_eq!(process(INPUT), 11048);
    }
}
