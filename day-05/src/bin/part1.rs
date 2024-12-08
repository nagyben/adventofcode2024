
use day_05::{is_correctly_ordered, page_rules_map, parse_input, PrintSequence};

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let (page_rules, print_sequences) = parse_input(input);

    // create a map of all the page rules like this:
    //
    // 29: [ 13, ],
    // 75: [ 29, 53, 47, 61, 13, ],
    // 61: [ 13, 53, 29, ],
    // 97: [ 13, 61, 47, 29, 53, 75, ],
    // 47: [ 53, 13, 61, 29, ],
    // 53: [ 29, 13, ],
    let page_rules_map = page_rules_map(&page_rules);

    // check every print sequence to see if it is correcly ordered
    print_sequences
        .iter()
        .filter(|ps| is_correctly_ordered(ps, &page_rules_map))
        .map(|ps| ps[ps.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
        assert_eq!(process(INPUT), 143);
    }
}
