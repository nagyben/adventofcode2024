use std::cmp::max;

use day_13::{parse_input, ButtonRule, Machine, Point};

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let machines = parse_input(input);
    // dbg!("{:?}", &machines);
    println!("target point: {:?}", &machines[0].prize);
    machines
        .iter()
        .fold(0, |acc, m| acc + get_cost(m).unwrap_or(0))
}

pub fn get_cost(machine: &Machine) -> Option<usize> {
    // find all points from origin to prize X and Y
    // using button A rules
    let min_points_a = max(
        machine.prize.x / machine.button_a.dx,
        machine.prize.y / machine.button_a.dy,
    ) + 1;
    let min_points_b = max(
        machine.prize.x / machine.button_b.dx,
        machine.prize.y / machine.button_b.dy,
    );
    let points_a: Vec<Point> = (0..=min_points_a)
        .map(|i| Point {
            x: i * machine.button_a.dx,
            y: i * machine.button_a.dy,
        })
        .collect();
    let points_b: Vec<Point> = (0..=min_points_b)
        .map(|i| Point {
            x: machine.prize.x - i * machine.button_b.dx,
            y: machine.prize.y - i * machine.button_b.dy,
        })
        .collect();
    let intersection = points_a
        .iter()
        .filter(|p| points_b.contains(p))
        .collect::<Vec<_>>();

    if intersection.len() == 0 {
        return None;
    }

    let num_a_presses = points_a.iter().position(|p| p == intersection[0]).unwrap();
    let num_b_presses = points_b.iter().position(|p| p == intersection[0]).unwrap();
    Some(3 * num_a_presses + num_b_presses)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        assert_eq!(process(INPUT), 480);
    }
}
