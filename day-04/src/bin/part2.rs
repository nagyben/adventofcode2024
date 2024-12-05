use day_04::parse_input;

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let parsed = parse_input(input);
    let mut num_xmas = 0;
    for y in 0..parsed.len() {
        for x in 0..parsed[y].len() {
            num_xmas += is_x_mas(x, y, &parsed) as usize;
        }
    }
    num_xmas
}

fn is_x_mas(x: usize, y: usize, parsed: &Vec<Vec<char>>) -> bool {
    // there are only 4 possible X-MAS combinations:
    // M.S   S.S   S.M   M.M
    // .A.   .A.   .A.   .A.
    // M.S   M.M   S.M   S.S
    // the fastest and easiest approach is to check a 3x3 subgrid around every coordinate, starting
    // with the A in the middle

    // check if we are at the edge
    if x == 0 || x == parsed[y].len() - 1 || y == 0 || y == parsed.len() - 1 {
        return false;
    }

    // check if the middle is an A
    if parsed[y][x] != 'A' {
        return false;
    }

    // check each corner position in this order:
    // top left, top right, bottom left, bottom right
    // then check if it matches one of the following sequences:
    // MSMS, SSMM, SMSM, MMSS
    let possible_sequences = vec![
        vec!['M', 'S', 'M', 'S'],
        vec!['S', 'S', 'M', 'M'],
        vec!['S', 'M', 'S', 'M'],
        vec!['M', 'M', 'S', 'S'],
    ];

    let corners: Vec<(isize, isize)> = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for sequence in possible_sequences {
        for (i, (dy, dx)) in corners.iter().enumerate() {
            if parsed[(y as isize + *dy) as usize][(x as isize + *dx) as usize] != sequence[i] {
                break;
            }
            if i == 3 {
                return true;
            }
        }
    }
    false
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
        assert_eq!(process(INPUT), 9);
    }
}
