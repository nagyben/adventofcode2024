use day_14::{calculate_safety_factor, parse_input};

fn main() {
    println!(
        "part2: {}",
        process(include_str!("../../input.txt"), 100, 102)
    );
}

fn process(input: &str, max_x: i32, max_y: i32) -> usize {
    let mut robots = parse_input(input).unwrap().1;
    let mut seconds = 0;
    // assuming that the number of robots has to be equal in all 4 quadrants
    // and that no robots are present in the middle parts
    let expected_safety_factor = (robots.len() / 4).pow(4);
    loop {
        robots.iter_mut().for_each(|robot| robot.tick(max_x, max_y));
        if calculate_safety_factor(&robots, max_x, max_y) == expected_safety_factor {
            break;
        }
        seconds += 1;
        if seconds % 1000 == 0 {
            println!("{}", seconds);
        }
    }
    seconds
}
