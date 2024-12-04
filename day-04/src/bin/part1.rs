use day_04::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

const SEARCH_WORD: &str = "XMAS";
const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    let mut num_xmas = 0;
    for y in 0..parsed.len() {
        for x in 0..parsed[y].len() {
            if parsed[y][x] == 'X' {
                num_xmas += count_xmas_for_coord(x, y, &parsed);
            }
        }
    }
    num_xmas
}

fn count_xmas_for_coord(x: usize, y: usize, parsed: &Vec<Vec<char>>) -> usize {
    DIRECTIONS
        .iter()
        .filter(|direction| check_direction(x, y, **direction, parsed))
        .count()
}

fn check_direction(x: usize, y: usize, direction: (isize, isize), parsed: &Vec<Vec<char>>) -> bool {
    let coords = (0..SEARCH_WORD.len())
        .map(|i| {
            (
                x as isize + i as isize * direction.1,
                y as isize + i as isize * direction.0,
            )
        })
        .collect::<Vec<(isize, isize)>>();

    // check if any of the coords are out of bounds
    if coords.iter().any(|(x, y)| {
        *x < 0 || *y < 0 || *y >= (parsed.len() as isize) || *x >= (parsed[0].len() as isize)
    }) {
        return false;
    }

    for (i, (x, y)) in coords.iter().enumerate() {
        if parsed[*y as usize][*x as usize] != SEARCH_WORD.chars().nth(i).unwrap() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(process(INPUT), 18);
    }
}
