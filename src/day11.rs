use crate::day11_luts::{LUTS_SOLN25, LUTS_SOLN75};
use rustc_hash::FxHashMap;

unsafe fn solve_for_n_memoised(n_iters: usize, content: &str, lookups: &[[usize; 1000]]) -> usize {
    let mut current = FxHashMap::default();

    let mut n = 0;
    for byte in content.bytes() {
        match byte {
            b'0'..b':' => {
                n *= 10;
                n += (byte - 48) as usize
            }
            _ => {
                *current.entry(n).or_insert(0) += 1;
                n = 0;
            }
        }
    }

    let mut next = FxHashMap::default();

    let mut total = 0;
    for n_iter in 0..n_iters {
        for (k, v) in &current {
            if n_iter < lookups.len() && *k < lookups.get_unchecked(n_iter).len() {
                total += lookups.get_unchecked(n_iter).get_unchecked(*k) * v;
                continue;
            }

            if *k == 0 {
                *next.entry(1).or_insert(0) += *v;
                continue;
            }
            let ilog = k.ilog10();
            if ilog % 2 == 0 {
                *next.entry(k * 2024).or_insert(0) += *v;
            } else {
                let divisor = 10usize.pow((ilog + 1) / 2);
                *next.entry(k % divisor).or_insert(0) += *v;
                *next.entry(k / divisor).or_insert(0) += *v;
            }
        }

        (current, next) = (next, current);
        next.drain();
    }

    total + current.values().sum::<usize>()
}

pub fn solve_for_n(n_iters: usize, content: &str) -> usize {
    let mut current = FxHashMap::default();

    let mut n = 0;
    for byte in content.bytes() {
        match byte {
            b'0'..b':' => {
                n *= 10;
                n += (byte - 48) as usize
            }
            _ => {
                *current.entry(n).or_insert(0) += 1;
                n = 0;
            }
        }
    }

    let mut next = FxHashMap::default();

    for _ in 0..n_iters {
        for (k, v) in &current {
            if *k == 0 {
                *next.entry(1).or_insert(0) += *v;
                continue;
            }
            let ilog = k.ilog10();
            if ilog % 2 == 0 {
                *next.entry(k * 2024).or_insert(0) += *v;
            } else {
                let divisor = 10usize.pow((ilog + 1) / 2);
                *next.entry(k % divisor).or_insert(0) += *v;
                *next.entry(k / divisor).or_insert(0) += *v;
            }
        }

        (current, next) = (next, current);
        next.drain();
    }

    current.values().sum()
}

pub fn part1(content: &str) -> usize {
    unsafe { solve_for_n_memoised(25, content, &LUTS_SOLN25) }
}

pub fn part2(content: &str) -> usize {
    unsafe { solve_for_n_memoised(75, content, &LUTS_SOLN75) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day11.txt");
        let result = part1(&contents);
        assert_eq!(result, 194_557);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day11.txt");
        let result = part2(&contents);
        assert_eq!(result, 231_532_558_973_909);
    }
}
