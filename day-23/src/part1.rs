use itertools::Itertools;

use petgraph::{prelude::UnGraphMap, visit::IntoNodeIdentifiers};

use crate::{graphviz, parse_input};

pub fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    let mut network = UnGraphMap::new();
    for connection in parsed {
        network.add_node(connection.0);
        network.add_node(connection.1);
        network.add_edge(connection.0, connection.1, ());
    }
    graphviz(&network);
    let nodes: Vec<&str> = network.node_identifiers().collect();
    let triples: Vec<(&str, &str, &str)> = nodes
        .into_iter()
        .tuple_combinations::<(_, _, _)>()
        .filter(|(a, b, c)| {
            network.contains_edge(a, b)
                && network.contains_edge(b, c)
                && network.contains_edge(c, a)
        })
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .collect();
    triples.len()
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
        assert_eq!(process(INPUT), 7);
    }
}
