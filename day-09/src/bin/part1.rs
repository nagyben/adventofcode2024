use day_09::{checksum, parse_input, Disk};

fn main() {
    println!("part1: {}", process(include_str!("../../input.txt")));
}

fn process(input: &str) -> usize {
    let mut disk = parse_input(input);
    compress(&mut disk);
    checksum(&disk)
}

fn compress(disk: &mut Disk) {
    let mut left_pointer = 0;
    let mut right_pointer = disk.len() - 1;
    while left_pointer < right_pointer {
        while disk[left_pointer].is_some() {
            left_pointer += 1;
        }
        while disk[right_pointer].is_none() {
            right_pointer -= 1;
        }
        if left_pointer >= right_pointer {
            break;
        }
        disk.swap(left_pointer, right_pointer);
    }
}

#[cfg(test)]
mod tests {
    use day_09::print_disk;

    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"2333133121414131402"#;
        assert_eq!(process(INPUT), 1928);
    }

    #[test]
    fn test_compress() {
        const INPUT: &str = r#"2333133121414131402"#;
        let mut disk = parse_input(INPUT);
        compress(&mut disk);
        assert_eq!(
            print_disk(&disk),
            "0099811188827773336446555566.............."
        );
    }
}
