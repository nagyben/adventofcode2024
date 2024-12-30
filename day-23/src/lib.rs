pub mod part1;
pub mod part2;

use std::{fs::File, io::Write};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::{
    dot::{Config, Dot},
    prelude::GraphMap,
};

pub fn parse_input(input: &str) -> Vec<(&str, &str)> {
    let v: IResult<&str, Vec<(&str, &str)>> =
        separated_list1(newline, separated_pair(alpha1, tag("-"), alpha1))(input);
    v.unwrap().1
}

pub fn graphviz<N, E, Ty>(graph: &GraphMap<N, E, Ty>)
where
    N: std::fmt::Debug + std::marker::Copy + std::hash::Hash + Eq + std::cmp::Ord,
    E: std::fmt::Debug,
    Ty: petgraph::EdgeType,
{
    let dot = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut file = File::create("graph.dot").expect("Unable to create file");
    file.write_all(dot.as_bytes())
        .expect("Unable to write data");
}
