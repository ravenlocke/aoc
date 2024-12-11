use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap};

const MAX_N: usize = 100;
const MAX_W: usize = MAX_N * MAX_N / 5;

use crate::utils::SmallVec;

fn parse_input(
    content: &str,
) -> (
    SmallVec<SmallVec<u8, MAX_N>, MAX_N>,
    SmallVec<(usize, usize), MAX_W>,
) {
    let mut grid = SmallVec::default();
    let mut nines = SmallVec::<(usize, usize), MAX_W>::default();

    let mut row = SmallVec::<u8, MAX_N>::default();
    let (mut i, mut j) = (0, 0);

    content.bytes().for_each(|byte| match byte {
        b'9' => {
            let val = byte - b'0';
            row.push(val);
            nines.push((i, j));
            j += 1;
        }
        b'0'..b'9' => {
            let val = byte - b'0';
            row.push(val);
            j += 1;
        }
        b'\n' => {
            grid.push(row.clone());
            row = SmallVec::<u8, 100>::default();
            i += 1;
            j = 0;
        }
        _ => {
            unreachable!()
        }
    });

    // Special case in case there's no newline at EOF.
    if row.len() != 0 {
        grid.push(row);
    }

    (grid, nines)
}

pub fn part1(content: &str) -> usize {
    let (grid, nines) = parse_input(content);
    let n_rows: usize = grid.len();
    let n_cols: usize = grid[0].len();
    let mut reachable_endpoints = FxHashMap::with_capacity_and_hasher(1_500, FxBuildHasher);

    for location in &nines {
        reachable_endpoints.insert(*location, vec![location.0 * n_cols + location.1]);
    }

    for idx in (1..10).rev() {
        let mut next_reachable_endpoints =
            FxHashMap::with_capacity_and_hasher(1_500, FxBuildHasher);
        for (loc, endpoints) in reachable_endpoints.iter() {
            let (i, j) = loc;

            if *i > 0 && grid[i - 1][*j] == idx - 1 {
                next_reachable_endpoints
                    .entry((i - 1, *j))
                    .or_insert(Vec::with_capacity(16))
                    .extend(endpoints);
            }

            if *i < n_cols - 1 && grid[i + 1][*j] == idx - 1 {
                next_reachable_endpoints
                    .entry((i + 1, *j))
                    .or_insert(Vec::with_capacity(16))
                    .extend(endpoints);
            }

            if *j > 0 && grid[*i][j - 1] == idx - 1 {
                next_reachable_endpoints
                    .entry((*i, j - 1))
                    .or_insert(Vec::with_capacity(16))
                    .extend(endpoints);
            }

            if *j < n_rows - 1 && grid[*i][j + 1] == idx - 1 {
                next_reachable_endpoints
                    .entry((*i, j + 1))
                    .or_insert(Vec::with_capacity(16))
                    .extend(endpoints);
            }
        }

        reachable_endpoints = next_reachable_endpoints;
    }

    let mut total = 0;
    for endpoints in reachable_endpoints.values() {
        total += endpoints.iter().unique().count()
    }

    total
}

pub fn part2(content: &str) -> usize {
    let (grid, nines) = parse_input(content);
    let n_rows: usize = grid.len();
    let n_cols: usize = grid[0].len();
    let mut reachable_endpoints = FxHashMap::default();
    let mut next_reachable_endpoints = FxHashMap::default();

    for location in &nines {
        reachable_endpoints.insert(*location, 1);
    }

    for idx in (1..10).rev() {
        for (loc, endpoints) in reachable_endpoints.iter() {
            let (i, j) = loc;

            if *i > 0 && grid[i - 1][*j] == idx - 1 {
                *next_reachable_endpoints.entry((i - 1, *j)).or_insert(0) += endpoints;
            }

            if *i < n_cols - 1 && grid[i + 1][*j] == idx - 1 {
                *next_reachable_endpoints.entry((i + 1, *j)).or_insert(0) += endpoints;
            }

            if *j > 0 && grid[*i][j - 1] == idx - 1 {
                *next_reachable_endpoints.entry((*i, j - 1)).or_insert(0) += endpoints;
            }

            if *j < n_rows - 1 && grid[*i][j + 1] == idx - 1 {
                *next_reachable_endpoints.entry((*i, j + 1)).or_insert(0) += endpoints;
            }
        }

        (reachable_endpoints, next_reachable_endpoints) =
            (next_reachable_endpoints, reachable_endpoints);
        next_reachable_endpoints.drain();
    }

    reachable_endpoints.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day10.txt");
        let result = part1(&contents);
        assert_eq!(result, 798);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day10.txt");
        let result = part2(&contents);
        assert_eq!(result, 1_816);
    }
}
