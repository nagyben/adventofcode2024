use day_09::{
    checksum, parse_contiguous, parse_input, print_contiguous, print_disk, ContiguousBlock,
    ContiguousBlockDisk, Disk,
};

fn main() {
    println!("part2: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    println!("{}", print_disk(&parse_input("222222222222222222222222")));
    let mut disk = parse_contiguous(input);
    defragment(&mut disk);
    println!("{:?}", &disk);
    let rdisk = contiguous_block_disk_to_disk(&disk);
    // println!("{}", print_disk(&rdisk));
    checksum(&rdisk)
}

fn contiguous_block_disk_to_disk(disk: &ContiguousBlockDisk) -> Disk {
    let mut result = Vec::new();
    for block in disk {
        match block {
            ContiguousBlock::Empty(size) => result.extend(vec![None; *size]),
            ContiguousBlock::File(id, size) => result.extend(vec![Some(*id); *size]),
        }
    }
    result
}

fn defragment(disk: &mut ContiguousBlockDisk) {
    let mut left_pointer = 0;
    let mut right_pointer = disk.len() - 1;
    while left_pointer < disk.len() {
        while let ContiguousBlock::File(_, _) = disk[left_pointer] {
            left_pointer += 1;
        }
        while let ContiguousBlock::Empty(_) = disk[right_pointer] {
            right_pointer -= 1;
        }
        println!("{}", print_contiguous(&disk));
        if let ContiguousBlock::Empty(empty_space) = disk[left_pointer] {
            if let ContiguousBlock::File(_, file_size) = disk[right_pointer] {
                if right_pointer <= left_pointer {
                    right_pointer = disk.len() - 1;
                    left_pointer += 1;
                }
                if empty_space >= file_size {
                    let file = disk.remove(right_pointer);
                    disk.insert(right_pointer, ContiguousBlock::Empty(file_size));
                    disk.insert(left_pointer, file);
                    left_pointer += 1;
                    if empty_space > file_size {
                        if left_pointer < disk.len() {
                            disk[left_pointer] = ContiguousBlock::Empty(empty_space - file_size);
                            merge_empty_space(disk);
                            // if let ContiguousBlock::Empty(e) = disk[left_pointer + 1] {
                            //     disk[left_pointer] = ContiguousBlock::Empty(empty_space + e);
                            //     disk.remove(left_pointer + 1);
                            // } else {
                            //     disk[left_pointer] =
                            //         ContiguousBlock::Empty(empty_space - file_size);
                            // }
                        }
                    } else if left_pointer < disk.len() {
                        disk.remove(left_pointer);
                    }
                    right_pointer = disk.len() - 1;
                } else {
                    right_pointer -= 1;
                }
            }
        } else {
            panic!("Invalid disk state");
        }
    }
}

fn merge_empty_space(disk: &mut ContiguousBlockDisk) {
    let mut index = 0;
    while index < disk.len() - 1 {
        if let ContiguousBlock::Empty(e) = disk[index] {
            if let ContiguousBlock::Empty(e2) = disk[index + 1] {
                disk[index] = ContiguousBlock::Empty(e + e2);
                disk.remove(index + 1);
                index -= 1;
            }
        }
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use day_09::{parse_contiguous, print_contiguous};

    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"2333133121414131402"#;
        assert_eq!(process(INPUT), 2858);
    }

    #[test]
    fn test_defragment() {
        const INPUT: &str = r#"2333133121414131402"#;
        let mut disk = parse_contiguous(INPUT);
        defragment(&mut disk);
        assert_eq!(
            print_contiguous(&disk),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }
}
