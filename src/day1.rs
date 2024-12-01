use rustc_hash::FxHashMap;
use std::{fs::File, io::Read};

fn parse_input(infile: &str) -> (Vec<i64>, Vec<i64>) {
    let mut buffer = String::new();
    File::open(infile)
        .expect("could not open file")
        .read_to_string(&mut buffer)
        .expect("");
    let mut list_a: Vec<i64> = Vec::new();
    let mut list_b: Vec<i64> = Vec::new();

    let mut populate_a = true;

    let mut n: i64 = 0;
    for char in buffer.chars() {
        if char.is_ascii_digit() {
            n *= 10;
            n += (char as i64) - 48
        } else if n != 0 {
            if populate_a {
                list_a.push(n)
            } else {
                list_b.push(n);
            }

            n = 0;
            populate_a ^= true
        }
    }

    (list_a, list_b)
}

pub fn part_one(infile: &str) -> i64 {
    let (mut a, mut b) = parse_input(infile);

    a.sort_unstable();
    b.sort_unstable();

    a.iter().zip(b.iter()).map(|(i, j)| (i - j).abs()).sum()
}

pub fn part_two(infile: &str) -> i64 {
    let (a, b) = parse_input(infile);
    let map = {
        let mut m = FxHashMap::default();
        b.iter().for_each(|i| *m.entry(*i).or_default() += 1);
        m
    };

    a.iter().map(|i| i * *map.get(i).get_or_insert(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let result = part_one("inputs/day01.txt");
        assert_eq!(result, 1_882_714);
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two("inputs/day01.txt");
        assert_eq!(result, 19_437_052);
    }
}
