use color_eyre::eyre::{eyre, Result};
use day_15::{parse_input, Grid, ObstacleType};

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let (mut grid, instructions, start_position) = parse_input(input);
    let mut robot_position = start_position;
    for instruction in instructions {
        if let Ok(()) = try_move(&mut grid, robot_position, instruction) {
            robot_position += instruction;
        }
    }
    calculate_gps_coordinates(&grid)
}

fn calculate_gps_coordinates(grid: &Grid) -> usize {
    let num_obstacles = grid
        .iter()
        .flatten()
        .filter(|o| matches!(o, ObstacleType::Box))
        .count();
    println!("{}", num_obstacles);
    grid.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().fold(0, |gps, (x, o)| {
            if let ObstacleType::Box = o {
                gps + 100 * y + x
            } else {
                gps
            }
        })
    })
}

fn print_grid(grid: &Grid, robot_position: Option<&glam::IVec2>) {
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if let Some(rp) = robot_position {
                if rp.x == x as i32 && rp.y == y as i32 {
                    print!("@");
                } else {
                    print!("{}", c);
                }
            } else {
                print!("{}", c);
            }
        });
        println!();
    })
}

fn try_move(grid: &mut Grid, current_location: glam::IVec2, direction: glam::IVec2) -> Result<()> {
    let target_location = current_location + direction;
    match grid[target_location.y as usize][target_location.x as usize] {
        ObstacleType::Wall => Err(eyre!(
            "Cannot move to {} because it is a wall",
            target_location,
        )),
        ObstacleType::Box => {
            let result = try_move(grid, target_location, direction);
            if let Ok(()) = result {
                grid[target_location.y as usize][target_location.x as usize] =
                    grid[current_location.y as usize][current_location.x as usize];
                grid[current_location.y as usize][current_location.x as usize] =
                    ObstacleType::Empty;
            }
            result
        }
        ObstacleType::Empty => {
            grid[target_location.y as usize][target_location.x as usize] =
                grid[current_location.y as usize][current_location.x as usize];
            grid[current_location.y as usize][current_location.x as usize] = ObstacleType::Empty;
            Ok(())
        }
        _ => panic!("encountered unknown obstacle type"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use day_15::parse_grid;

    #[test]
    fn example() {
        const input: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        assert_eq!(process(input), 2028);
    }

    #[test]
    fn example_2() {
        const INPUT: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        assert_eq!(process(INPUT), 10092);
    }

    #[test]
    fn gps_coords_1() {
        const INPUT: &str = r#"#######
#...O..
#......"#;
        let grid = parse_grid(INPUT).unwrap().1;
        assert_eq!(calculate_gps_coordinates(&grid), 104);
    }

    #[test]
    fn gps_coords_2() {
        const INPUT: &str = r#"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########"#;
        let grid = parse_grid(INPUT).unwrap().1;
        print_grid(&grid, Some(&glam::IVec2 { x: 0, y: 0 }));
        assert_eq!(calculate_gps_coordinates(&grid), 2028);
    }
}
