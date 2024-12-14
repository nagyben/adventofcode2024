use day_14::{calculate_safety_factor, parse_input, Robot, Robots};

fn main() {
    println!(
        "part1: {}",
        process(include_str!("../../input.txt"), 100, 102)
    );
}

fn process(input: &str, max_x: i32, max_y: i32) -> usize {
    let mut robots = parse_input(input).unwrap().1;
    for _ in 0..100 {
        robots.iter_mut().for_each(|robot| robot.tick(max_x, max_y));
    }
    calculate_safety_factor(&robots, max_x, max_y)
}

fn print_robots(robots: &Vec<Robot>, max_x: i32, max_y: i32) {
    let mut grid = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];

    robots
        .iter()
        .for_each(|robot| grid[robot.position.y as usize][robot.position.x as usize] += 1);
    grid.iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|c| if *c > 0 {
                    format!("{}", c)
                } else {
                    String::from(".")
                })
                .collect::<String>()
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        assert_eq!(process(INPUT, 10, 6), 12);
    }
}
