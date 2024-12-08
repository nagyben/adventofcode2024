use std::collections::HashSet;

use day_06::{parse_input, Grid, GuardDirection, StartPosition};

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

// store the directions that a guard has faced in each cell
// each cell contains a set of directions
// if the guard hasn't visited the cell then the set will be empty
type DirectionalGrid = Vec<Vec<HashSet<GuardDirection>>>;

fn process(input: &str) -> usize {
    let (grid, start_position, start_direction) = parse_input(input);
    let mut directional_grid = grid
        .iter()
        .map(|row| row.iter().map(|_| HashSet::new()).collect())
        .collect();
    fill_guard_path_directional(
        &grid,
        &mut directional_grid,
        &start_position,
        &start_direction,
    );
    println!("{:?}", directional_grid);

    // now we count the number of clockwise intersections which aren't at the edge of the grid
    directional_grid
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != 0 && *i != directional_grid.len() - 1) // ignore the first and last row
        .map(|(_, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, _)| *j != 0 && *j != row.len() - 1) // ignore the first and last column
                .filter(|(_, cell)| {
                    (cell.contains(&GuardDirection::Up) && cell.contains(&GuardDirection::Right))
                        || (cell.contains(&GuardDirection::Right)
                            && cell.contains(&GuardDirection::Down))
                        || (cell.contains(&GuardDirection::Left)
                            && cell.contains(&GuardDirection::Up))
                        || (cell.contains(&GuardDirection::Down)
                            && cell.contains(&GuardDirection::Left))
                })
                .count()
        })
        .sum()
}

fn fill_guard_path_directional(
    grid: &Grid,
    directional_grid: &mut DirectionalGrid,
    start_position: &StartPosition,
    start_direction: &GuardDirection,
) {
    let (mut x, mut y) = *start_position;
    let grid_size = (grid.len() - 1, grid[0].len() - 1);
    let mut direction = *start_direction;
    loop {
        directional_grid[y][x].insert(direction);
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
        assert_eq!(process(INPUT), 6);
    }
}
