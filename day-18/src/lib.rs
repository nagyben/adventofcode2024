use pathfinding::prelude::dijkstra;

pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> Vec<glam::IVec2> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().trim().parse().unwrap();
            glam::IVec2::new(x, y)
        })
        .collect()
}

pub fn find_exit_path(
    corrupted_memory_order: &[glam::IVec2],
    num_corruptions: usize,
    grid_size: usize,
) -> Option<(Vec<glam::IVec2>, isize)> {
    const DIRECTIONS: [glam::IVec2; 4] = [
        glam::IVec2::new(0, 1),
        glam::IVec2::new(0, -1),
        glam::IVec2::new(1, 0),
        glam::IVec2::new(-1, 0),
    ];
    dijkstra(
        &glam::IVec2::new(0, 0),
        |&pos| {
            DIRECTIONS
                .iter()
                .map(move |&dir| pos + dir)
                .filter(|&pos| {
                    pos.x >= 0
                        && pos.x <= grid_size as i32
                        && pos.y >= 0
                        && pos.y <= grid_size as i32
                        && !corrupted_memory_order[..num_corruptions].contains(&pos)
                })
                .map(move |pos| (pos, 1))
        },
        |pos| *pos == glam::IVec2::new(grid_size as i32, grid_size as i32),
    )
}
