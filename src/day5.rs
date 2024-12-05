use std::cmp::Ordering;

use itertools::Itertools;

fn parse_radix_10(input: &[u8]) -> usize {
    let mut n = 0;
    for i in input {
        n *= 10;
        n += *i as usize
    }
    n -= 528;

    n
}

pub fn part1(content: &str) -> u64 {
    let mut rels = [[Ordering::Equal; 128]; 128];
    for i in 0..1176 {
        let idx = i * 6;
        let a = parse_radix_10(&content.as_bytes()[idx..idx + 2]);
        let b = parse_radix_10(&content.as_bytes()[idx + 3..idx + 5]);

        rels[a][b] = Ordering::Less;
        rels[b][a] = Ordering::Greater;
    }

    content[1176 * 6 + 1..]
        .lines()
        .filter_map(|i| {
            if i.split(',')
                .map(|i| parse_radix_10(i.as_bytes()))
                .tuple_windows()
                .all(|(a, b)| rels[a][b] == Ordering::Less)
            {
                return Some(parse_radix_10(i[i.len() / 2 - 1..i.len() / 2 + 1].as_bytes()) as u64);
            }
            None
        })
        .sum::<u64>()
}

pub fn part2(content: &str) -> u64 {
    let mut rels = [[Ordering::Equal; 128]; 128];
    for i in 0..1176 {
        let idx = i * 6;
        let a = parse_radix_10(&content.as_bytes()[idx..idx + 2]);
        let b = parse_radix_10(&content.as_bytes()[idx + 3..idx + 5]);

        rels[a][b] = Ordering::Less;
        rels[b][a] = Ordering::Greater;
    }

    content[1176 * 6 + 1..]
        .lines()
        .filter_map(|i| {
            let mut x: Vec<_> = i.split(',').map(|i| parse_radix_10(i.as_bytes())).collect();

            if x.iter()
                .tuple_windows()
                .all(|(a, b)| rels[*a][*b] == Ordering::Less)
            {
                return None;
            }

            x.sort_by(|a, b| rels[*a][*b]);

            Some(x[x.len() / 2] as u64)
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day05.txt");
        let result = part1(&contents);
        assert_eq!(result, 7_024);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day05.txt");
        let result = part2(&contents);
        assert_eq!(result, 4_151);
    }
}
