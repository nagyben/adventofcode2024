use day_09::{parse_input, Disk};

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

fn checksum(disk: &Disk) -> usize {
    disk.iter().enumerate().fold(0, |acc, (i, block)| {
        if let Some(id) = block {
            acc + i * id
        } else {
            acc
        }
    })
}

fn print_disk(disk: &Disk) -> String {
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

#[cfg(test)]
mod tests {
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
