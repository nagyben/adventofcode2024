pub type Disk = Vec<Option<usize>>;
pub fn parse_input(input: &str) -> Disk {
    let mut file = true;
    let mut id = 0;
    let mut disk: Vec<Option<usize>> = vec![];
    for char in input.chars() {
        if let Some(n_blocks) = char.to_digit(10) {
            if file {
                for _ in 0..n_blocks {
                    disk.push(Some(id))
                }
                id += 1;
                file = false;
            } else {
                for _ in 0..n_blocks {
                    disk.push(None)
                }
                file = true;
            }
        }
    }
    disk
}
