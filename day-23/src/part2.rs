use itertools::Itertools;
use petgraph::{prelude::UnGraphMap, visit::IntoNodeIdentifiers};

use crate::{graphviz, parse_input};

pub fn process(input: &str) -> String {
    let parsed = parse_input(input);
    let mut network = UnGraphMap::new();
    for connection in parsed {
        network.add_node(connection.0);
        network.add_node(connection.1);
        network.add_edge(connection.0, connection.1, ());
    }
    graphviz(&network);
    // every node in the graph has 13 connections apparently
    let num_connections = network
        .node_identifiers()
        .map(|node| network.neighbors(node).count())
        .max()
        .unwrap();
    // .collect()
    println!("{:?}", num_connections);

    let nodes: Vec<&str> = network.node_identifiers().collect();
    let cliques = nodes
        .iter()
        .flat_map(|&node| {
            let mut group = vec![node];
            group.extend(network.neighbors(node).clone());
            group
                .into_iter()
                .combinations(num_connections - 1)
                .map(|c| c.into_iter().sorted().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .counts();
    let max_value = cliques.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    cliques
        .iter()
        .filter(|(_, v)| *v == max_value)
        .flat_map(|(k, _)| k)
        .unique()
        .sorted()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
        assert_eq!(process(INPUT), "co,de,ka,ta");
    }
}
