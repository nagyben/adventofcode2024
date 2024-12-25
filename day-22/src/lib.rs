pub mod part1;
pub mod part2;

pub fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn generate_n_secrets(mut secret: usize, n: usize) -> Vec<usize> {
    (0..n)
        .map(|_| {
            secret = generate(secret);
            secret
        })
        .collect()
}

fn generate(mut secret: usize) -> usize {
    // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
    let v1 = secret * 64;
    secret = mix(secret, v1);
    secret = prune(secret);
    let v2 = secret / 32;
    secret = mix(secret, v2);
    secret = prune(secret);
    let v3 = secret * 2048;
    secret = mix(secret, v3);
    secret = prune(secret);
    secret
}

fn mix(secret: usize, value: usize) -> usize {
    secret ^ value
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[test]
    fn test_generate() {
        let mut secret = 123;
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let actual: Vec<usize> = expected
            .iter()
            .map(|_| {
                secret = generate(secret);
                secret
            })
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37)
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920)
    }

    #[rstest]
    #[case(1, 8685429)]
    #[case(10, 4700978)]
    #[case(100, 15273692)]
    #[case(2024, 8667524)]
    fn test_generate_nth_secret(#[case] secret: usize, #[case] expected: usize) {
        let actual = generate_n_secrets(secret, 2000);
        assert_eq!(*actual.last().unwrap(), expected);
    }
}
