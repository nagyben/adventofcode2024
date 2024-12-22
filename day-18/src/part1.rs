use crate::{find_exit_path, parse_input};

pub fn process(input: &str) -> isize {
    let corrupted_memory_order = parse_input(input);
    find_exit_path_length(&corrupted_memory_order, 1024, 70)
}

fn find_exit_path_length(
    corrupted_memory_order: &[glam::IVec2],
    num_corruptions: usize,
    grid_size: usize,
) -> isize {
    find_exit_path(corrupted_memory_order, num_corruptions, grid_size)
        .unwrap()
        .1
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
        let num_corruptions = 12;
        let corruped_memory_order = parse_input(INPUT)[0..num_corruptions].to_vec();
        let grid_size = 6;
        let path_length = find_exit_path_length(&corruped_memory_order, 12, grid_size);
        assert_eq!(path_length, 22); // number of steps, not number of visited locations
    }
}
