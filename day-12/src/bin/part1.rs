use petgraph::algo::condensation;
use petgraph::graph::NodeIndex;
use petgraph::visit::IntoNodeReferences;
use petgraph::Graph;
use petgraph::{dot::Dot, graphmap::GraphMap, prelude::UnGraphMap};
use std::io::Write;
use std::{collections::HashMap, fs::File};

use day_12::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let garden = parse_input(input);
    let grid_map: HashMap<(i32, i32), char> = garden
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| ((x as i32, y as i32), c))
        })
        .collect();

    let mut graph: UnGraphMap<(i32, i32), ()> = GraphMap::new();

    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for ((x, y), &c) in &grid_map {
        graph.add_node((*x, *y));
        for (dx, dy) in &DIRECTIONS {
            let nx = x + dx;
            let ny = y + dy;
            if let Some(&nc) = grid_map.get(&(nx, ny)) {
                if c == nc {
                    graph.add_edge((*x, *y), (nx, ny), ());
                }
            }
        }
    }
    let condensed_graph = condensation(graph.clone().into_graph::<NodeIndex>(), false);
    condensed_graph
        .node_references()
        .fold(0, |price, (_, nodes)| {
            let area = nodes.len();
            let perimeter = nodes
                .iter()
                .fold(0, |acc, n| acc + 4 - graph.neighbors(*n).count());
            price + area * perimeter
        })
}

fn graphviz(graph: &Graph<(i32, i32), ()>) {
    let mut file = File::create("graph.dot").expect("Unable to create file");
    file.write_all(format!("{:?}", Dot::with_config(&graph, &[])).as_bytes())
        .expect("Unable to write data");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        const INPUT: &str = r#"AAAA
BBCD
BBCC
EEEC"#;
        assert_eq!(process(INPUT), 140);
    }

    #[test]
    fn example_2() {
        const INPUT: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        assert_eq!(process(INPUT), 1930);
    }

    #[test]
    fn example_3() {
        const INPUT: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"#;
        assert_eq!(process(INPUT), 772);
    }
}
