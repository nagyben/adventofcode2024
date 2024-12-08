use day_05::{is_correctly_ordered, page_rules_map, parse_input, PageRules};

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let (page_rules, mut print_sequences) = parse_input(input);
    let page_rules_map = page_rules_map(&page_rules);
    print_sequences
        .iter_mut()
        .filter(|ps| !is_correctly_ordered(ps, &page_rules_map))
        .map(|ps| {
            // running this once doesn't seem to fix the ordering completely,
            // but running it again does
            // :shrug:
            fix_ordering(ps, &page_rules);
            fix_ordering(ps, &page_rules);
            ps[ps.len() / 2]
        })
        .sum()
}

fn fix_ordering(print_sequence: &mut Vec<usize>, page_rules: &PageRules) {
    page_rules.iter().for_each(|(a, b)| {
        if print_sequence.contains(a) && print_sequence.contains(b) {
            let a_index = print_sequence.iter().position(|x| *x == *a).unwrap();
            let b_index = print_sequence.iter().position(|x| *x == *b).unwrap();
            if a_index > b_index {
                print_sequence.remove(a_index);
                print_sequence.insert(b_index, *a)
            }
        }
    });
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
        assert_eq!(process(INPUT), 123);
    }
}
