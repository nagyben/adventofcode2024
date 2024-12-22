use crate::{find_exit_path, parse_input};

pub fn process(input: &str) -> String {
    let corrupted_memory_order = parse_input(input);
    let coord = find_blocking_corruption_coordinate(&corrupted_memory_order, 70);
    format!("{}", coord)
}

fn find_blocking_corruption_coordinate(
    corrupted_memory_order: &[glam::IVec2],
    grid_size: usize,
) -> glam::IVec2 {
    let mut num_corruptions = 1024;
    loop {
        if find_exit_path(corrupted_memory_order, num_corruptions, grid_size).is_none() {
            return corrupted_memory_order[num_corruptions - 1];
        }
        num_corruptions += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
        let corrupted_memory_order = parse_input(INPUT);
        assert_eq!(
            find_blocking_corruption_coordinate(&corrupted_memory_order, 6),
            glam::IVec2::new(6, 1)
        );
    }
}
