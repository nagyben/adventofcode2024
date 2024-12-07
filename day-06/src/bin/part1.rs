use day_06::{parse_input, Grid, GuardDirection, StartPosition};

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let (mut grid, start_position, start_direction) = parse_input(input);
    fill_guard_path(&mut grid, &start_position, &start_direction);
    // count all the Xs in the grid
    grid.iter().flatten().filter(|&&ch| ch == 'X').count()
}

fn fill_guard_path(
    grid: &mut Grid,
    start_position: &StartPosition,
    start_direction: &GuardDirection,
) {
    let (mut x, mut y) = *start_position;
    let grid_size = (grid.len() - 1, grid[0].len() - 1);
    let mut direction = *start_direction;
    loop {
        grid[y][x] = 'X';

        match direction {
            GuardDirection::Up => {
                if y == 0 {
                    break;
                }
                if grid[y - 1][x] == '#' {
                    direction = GuardDirection::Right;
                } else {
                    y -= 1;
                }
            }
            GuardDirection::Down => {
                if y == grid_size.0 {
                    break;
                }
                if grid[y + 1][x] == '#' {
                    direction = GuardDirection::Left;
                } else {
                    y += 1;
                }
            }
            GuardDirection::Left => {
                if x == 0 {
                    break;
                }
                if grid[y][x - 1] == '#' {
                    direction = GuardDirection::Up;
                } else {
                    x -= 1;
                }
            }
            GuardDirection::Right => {
                if x == grid_size.1 {
                    break;
                }
                if grid[y][x + 1] == '#' {
                    direction = GuardDirection::Down;
                } else {
                    x += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        assert_eq!(process(INPUT), 41);
    }
}
