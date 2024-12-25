use std::collections::HashMap;

use crate::{generate_n_secrets, parse_input};
use plotters::prelude::*;

pub fn process(input: &str) -> usize {
    let secrets = parse_input(input);
    let pricess = get_prices(&secrets, 2000);
    let deltass = get_deltas(&pricess);
    let buyers_deltas = deltass
        .iter()
        .map(|deltas| deltas.windows(4).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut bananas: HashMap<Vec<isize>, Vec<Option<usize>>> = HashMap::new();
    for (buyer_id, buyer) in buyers_deltas.iter().enumerate() {
        for (price_id, window) in buyer.iter().enumerate() {
            let price = pricess[buyer_id][price_id + 3];
            let h = bananas
                .entry(window.to_vec())
                .or_insert(vec![None; pricess.len()]);
            if h[buyer_id].is_none() {
                h[buyer_id] = Some(price);
            }
        }
    }
    let best_sequence = bananas
        .iter()
        .max_by(|a, b| {
            a.1.iter()
                .map(|x| x.unwrap_or(0))
                .sum::<usize>()
                .cmp(&b.1.iter().map(|x| x.unwrap_or(0)).sum::<usize>())
        })
        .unwrap();
    best_sequence.1.iter().flatten().sum()
}

fn get_deltas(pricess: &[Vec<usize>]) -> Vec<Vec<isize>> {
    let deltass: Vec<_> = pricess
        .iter()
        .map(|prices| {
            let mut p = vec![0];
            p.extend(
                prices
                    .windows(2)
                    .map(|p| p[1] as isize - p[0] as isize)
                    .collect::<Vec<isize>>(),
            );
            p
        })
        .collect();
    deltass
}

fn get_prices(secrets: &[usize], n_prices: usize) -> Vec<Vec<usize>> {
    secrets
        .iter()
        .map(|secret| {
            let mut prices = vec![*secret];
            prices.extend(generate_n_secrets(*secret, n_prices - 1));
            prices
                .iter()
                .map(|secret| secret % 10)
                .collect::<Vec<usize>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        const INPUT: &str = r#"1
2
3
2024"#;
        assert_eq!(process(INPUT), 23);
    }

    #[test]
    fn test_prices() {
        //      123: 3
        // 15887950: 0 (-3)
        // 16495136: 6 (6)
        //   527345: 5 (-1)
        //   704524: 4 (-1)
        //  1553684: 4 (0)
        // 12683156: 6 (2)
        // 11100544: 4 (-2)
        // 12249484: 4 (0)
        //  7753432: 2 (-2)
        let secret = 123;
        let prices = get_prices(&[secret], 10);
        assert_eq!(prices[0], vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }
    #[test]
    fn test_deltas() {
        //      123: 3
        // 15887950: 0 (-3)
        // 16495136: 6 (6)
        //   527345: 5 (-1)
        //   704524: 4 (-1)
        //  1553684: 4 (0)
        // 12683156: 6 (2)
        // 11100544: 4 (-2)
        // 12249484: 4 (0)
        //  7753432: 2 (-2)
        let secret = 123;
        let prices = get_prices(&[secret], 10);
        let deltas = get_deltas(&prices);
        assert_eq!(deltas[0], vec![0, -3, 6, -1, -1, 0, 2, -2, 0, -2]);
    }
}
