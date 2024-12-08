use std::collections::HashMap;

use day_08::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    let antenna_coords = get_antenna_coords(&parsed);
    println!("{:?}", antenna_coords);
    todo!()
}

fn get_antenna_coords(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antenna_coords = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_alphanumeric() {
                antenna_coords
                    .entry(*cell)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
    }
    antenna_coords
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;
        assert_eq!(process(INPUT), 14);
    }
}
