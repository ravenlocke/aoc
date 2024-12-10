// TODO - Rework to reduce number of allocations.

use itertools::Itertools;
use rustc_hash::FxHashMap;

const N: usize = 57;
const M: usize = N;

fn parse_input(content: &str) -> ([[u8; M]; N], FxHashMap<u8, Vec<(usize, usize)>>) {
    let mut grid = [[0; M]; N];
    let mut locations = FxHashMap::<u8, Vec<(usize, usize)>>::default();

    (0..N).for_each(|i| {
        (0..M).for_each(|j| {
            let val = content.as_bytes()[i * (N + 1) + j] - b'0';
            grid[i][j] = val;
            locations.entry(val).or_insert(Vec::new()).push((i, j));
        });
    });

    (grid, locations)
}

pub fn part1(content: &str) -> usize {
    let (grid, locations) = parse_input(content);
    let mut reachable_endpoints = FxHashMap::<(usize, usize), Vec<(usize, usize)>>::default();

    for location in locations.get(&9).expect("TODO") {
        reachable_endpoints.insert(*location, vec![*location]);
    }

    for idx in (1..10).rev() {
        for loc in locations.get(&idx).unwrap() {
            let (i, j) = loc;

            let endpoints = reachable_endpoints.get(loc).unwrap_or(&vec![]).clone();

            if *i > 0 && grid[i - 1][*j] == idx - 1 {
                reachable_endpoints
                    .entry((i - 1, *j))
                    .or_insert(vec![])
                    .extend(&endpoints);
            }

            if *i < M - 1 && grid[i + 1][*j] == idx - 1 {
                reachable_endpoints
                    .entry((i + 1, *j))
                    .or_insert(vec![])
                    .extend(&endpoints);
            }

            if *j > 0 && grid[*i][j - 1] == idx - 1 {
                reachable_endpoints
                    .entry((*i, j - 1))
                    .or_insert(vec![])
                    .extend(&endpoints);
            }

            if *j < N - 1 && grid[*i][j + 1] == idx - 1 {
                reachable_endpoints
                    .entry((*i, j + 1))
                    .or_insert(vec![])
                    .extend(&endpoints);
            }
        }
    }

    let mut total = 0;
    for location in locations.get(&0).expect("") {
        if let Some(endpoints) = reachable_endpoints.get(location) {
            total += endpoints.iter().unique().count()
        }
    }

    total
}

pub fn part2(content: &str) -> usize {
    let (grid, locations) = parse_input(content);
    let mut reachable_endpoints = FxHashMap::<(usize, usize), Vec<(usize, usize)>>::default();

    for location in locations.get(&9).expect("TODO") {
        reachable_endpoints.insert(*location, vec![*location]);
    }

    for idx in (1..10).rev() {
        for loc in locations.get(&idx).unwrap() {
            let (i, j) = loc;

            let endpoints = reachable_endpoints.get(loc).unwrap_or(&vec![]).clone();

            if *i > 0 && grid[i - 1][*j] == idx - 1 {
                reachable_endpoints
                    .entry((i - 1, *j))
                    .or_insert(vec![])
                    .extend(&endpoints);
            }

            if *i < M - 1 && grid[i + 1][*j] == idx - 1 {
                reachable_endpoints
                    .entry((i + 1, *j))
                    .or_insert(vec![])
                    .extend(&endpoints);
            }

            if *j > 0 && grid[*i][j - 1] == idx - 1 {
                reachable_endpoints
                    .entry((*i, j - 1))
                    .or_insert(vec![])
                    .extend(&endpoints);
            }

            if *j < N - 1 && grid[*i][j + 1] == idx - 1 {
                reachable_endpoints
                    .entry((*i, j + 1))
                    .or_insert(vec![])
                    .extend(&endpoints);
            }
        }
    }

    let mut total = 0;
    for location in locations.get(&0).expect("") {
        if let Some(endpoints) = reachable_endpoints.get(location) {
            total += endpoints.iter().count()
        }
    }

    total
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
