pub enum Tile {
    Wall,
    Empty,
    End,
}
pub type Maze = Vec<Vec<Tile>>;

pub fn parse_input(input: &str) -> (Maze, glam::IVec2) {
    let mut start_pos = glam::IVec2::new(0, 0);
    let maze = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'E' => Tile::End,
                    'S' => {
                        start_pos = glam::IVec2::new(x as i32, y as i32);
                        Tile::Empty
                    }
                    _ => panic!("Invalid character in input: {}", c),
                })
                .collect()
        })
        .collect();
    (maze, start_pos)
}
