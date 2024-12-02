use day02::is_safe;
use day02::parse_input;

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    parsed
        .iter()
        .map(|report| {
            dampened_reports(report)
                .iter()
                .any(|dampened_report| is_safe(dampened_report.clone())) as usize
        })
        .sum()
}

fn dampened_reports(report: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut dampened_reports = Vec::new();
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        dampened_reports.push(new_report);
    }
    dampened_reports
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(process(INPUT), 4);
    }
}
