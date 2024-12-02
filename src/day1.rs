/// A solution to day one of AOC 2024.
///
/// This code makes optimisations based on the following input observations:
///     - Each line in the input is of the form r"\d{5}\s{3}\d{5}\n" (i.e.,
///       this solution WILL NOT work on the example given in the description).
///     - There are 1,000 entries in the input.
use rustc_hash::{FxBuildHasher, FxHashMap};

const CAPACITY: usize = 1_000;

struct NumberIter<'a> {
    text: &'a str,
    pos: usize,
    extra_skip: bool,
}

impl Iterator for NumberIter<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + 5 > self.text.len() {
            return None;
        }
        let mut n = 0i64;

        for c in &self.text.as_bytes()[self.pos..self.pos + 5] {
            n *= 10;
            n += *c as i64;
        }
        n -= 533_328;

        if self.extra_skip {
            self.pos += 8;
            self.extra_skip = false;
        } else {
            self.pos += 6;
            self.extra_skip = true;
        }
        Some(n)
    }
}

impl NumberIter<'_> {
    fn new(text: &str) -> NumberIter {
        NumberIter {
            text,
            pos: 0,
            extra_skip: true,
        }
    }
}

#[inline(always)]
fn parse_input_part1(contents: &str) -> ([i64; CAPACITY], [i64; CAPACITY]) {
    let mut arr_a = [0; CAPACITY];
    let mut arr_b = [0; CAPACITY];

    let number_iter = NumberIter::new(contents);
    for (idx, number) in number_iter.enumerate() {
        if idx % 2 == 0 {
            arr_a[idx / 2] = number;
        } else {
            arr_b[idx / 2] = number;
        }
    }

    (arr_a, arr_b)
}

#[inline(always)]
fn parse_input_part2(contents: &str) -> ([i64; CAPACITY], FxHashMap<i64, i64>) {
    let mut arr_a = [0; CAPACITY];
    let mut b_count_map = FxHashMap::with_capacity_and_hasher(CAPACITY, FxBuildHasher);

    let number_iter = NumberIter::new(contents);
    for (idx, number) in number_iter.enumerate() {
        if idx % 2 == 0 {
            arr_a[idx / 2] = number;
        } else {
            *b_count_map.entry(number).or_default() += 1;
        }
    }

    (arr_a, b_count_map)
}

pub fn part1(contents: &str) -> i64 {
    let (mut a, mut b) = parse_input_part1(contents);

    a.sort_unstable();
    b.sort_unstable();

    a.iter().zip(b.iter()).map(|(i, j)| (i - j).abs()).sum()
}

pub fn part2(contents: &str) -> i64 {
    let (a, b) = parse_input_part2(contents);
    a.iter().map(|i| i * *b.get(i).get_or_insert(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day01.txt");
        let result = part1(&contents);
        assert_eq!(result, 1_882_714);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day01.txt");
        let result = part2(contents);
        assert_eq!(result, 19_437_052);
    }
}
