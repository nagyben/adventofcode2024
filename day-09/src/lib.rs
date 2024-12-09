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

pub type ContiguousBlockDisk = Vec<ContiguousBlock>;
pub type ID = usize;
pub type BlockSize = usize;

#[derive(Clone, Eq, Debug, PartialEq)]
pub enum ContiguousBlock {
    Empty(BlockSize),
    File(ID, BlockSize),
}

pub fn parse_contiguous(input: &str) -> ContiguousBlockDisk {
    let mut file = true;
    let mut id = -1;
    input
        .trim()
        .chars()
        .map(|c| {
            if let Some(n_blocks) = c.to_digit(10) {
                if file {
                    file = false;
                    id += 1;
                    ContiguousBlock::File(id as usize, n_blocks as usize)
                } else {
                    file = true;
                    ContiguousBlock::Empty(n_blocks as usize)
                }
            } else {
                panic!("Invalid input")
            }
        })
        .collect()
}

pub fn checksum(disk: &Disk) -> usize {
    disk.iter().enumerate().fold(0, |acc, (i, block)| {
        if let Some(id) = block {
            acc + i * id
        } else {
            acc
        }
    })
}

pub fn print_disk(disk: &Disk) -> String {
    disk.iter()
        .map(|block| {
            if let Some(id) = block {
                format!("{}", id)
            } else {
                ".".to_string()
            }
        })
        .collect()
}

pub fn print_contiguous(disk: &ContiguousBlockDisk) -> String {
    disk.iter()
        .map(|block| match block {
            ContiguousBlock::Empty(size) => ".".repeat(*size),
            ContiguousBlock::File(id, size) => format!("{}", id).repeat(*size),
        })
        .collect()
}
