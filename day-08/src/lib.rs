use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn get_antenna_coords(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antenna_coords = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_alphanumeric() {
                antenna_coords
                    .entry(*cell)
                    .or_insert_with(Vec::new)
                    .push((y, x));
            }
        }
    }
    antenna_coords
}

pub fn print_antinodes(antinodes: &Vec<Vec<bool>>) {
    for row in antinodes.iter() {
        for cell in row.iter() {
            print!("{}", if *cell { "X" } else { "." });
        }
        println!();
    }
}
