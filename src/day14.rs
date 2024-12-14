use std::{isize, iter};

const M: isize = 103;
const N: isize = 101;
const K: isize = 100;

const MID_M: isize = M / 2;
const MID_N: isize = N / 2;

const MAX_STATES: isize = M * N;

pub fn part1(content: &str) -> isize {
    let mut digits = [0, 0, 0, 0];
    let mut pointer = 0usize;
    let mut n = 0;
    let mut is_neg = false;

    let mut quadrants = [0, 0, 0, 0];

    content.bytes().for_each(|byte| match byte {
        b'0'..b':' => {
            n *= 10;
            n += (byte - b'0') as isize;
        }
        b'\n' => {
            if is_neg {
                n *= -1
            }
            digits[pointer] = n;

            let (col, row) = (
                (digits[0] + K * digits[2]).rem_euclid(N),
                (digits[1] + K * digits[3]).rem_euclid(M),
            );

            if row > MID_M {
                if col > MID_N {
                    quadrants[0] += 1;
                } else if col < MID_N {
                    quadrants[1] += 1;
                } else {
                }
            } else if row < MID_M {
                if col > MID_N {
                    quadrants[2] += 1;
                } else if col < MID_N {
                    quadrants[3] += 1;
                } else {
                }
            } else {
            }

            n = 0;
            is_neg = false;
            pointer = 0;
            digits = [0, 0, 0, 0];
        }
        b'-' => {
            is_neg = true;
        }
        b' ' | b',' => {
            if is_neg {
                n *= -1
            }
            digits[pointer] = n;

            n = 0;
            is_neg = false;
            pointer += 1;
        }
        _ => {}
    });

    quadrants.iter().product()
}

pub fn part2(content: &str) -> isize {
    let mut digits = [0, 0, 0, 0];
    let mut pointer = 0usize;
    let mut n = 0;
    let mut is_neg = false;

    let mut states = Vec::with_capacity(MAX_STATES as usize);
    let mut deltas = Vec::with_capacity(MAX_STATES as usize);
    let mut closest_dist = usize::MAX;
    let mut closest_iter = isize::MAX;

    content.bytes().for_each(|byte| match byte {
        b'0'..b':' => {
            n *= 10;
            n += (byte - b'0') as isize;
        }
        b'\n' => {
            if is_neg {
                n *= -1
            }
            digits[pointer] = n;

            states.push((digits[0], digits[1]));
            deltas.push((digits[2], digits[3]));

            n = 0;
            is_neg = false;
            pointer = 0;
            digits = [0, 0, 0, 0];
        }
        b'-' => {
            is_neg = true;
        }
        b' ' | b',' => {
            if is_neg {
                n *= -1
            }
            digits[pointer] = n;

            n = 0;
            is_neg = false;
            pointer += 1;
        }
        _ => {}
    });

    for n_iter in 1..MAX_STATES + 1 {
        (&mut states).iter_mut().zip(&deltas).for_each(|(i, j)| {
            i.0 = (i.0 + j.0).rem_euclid(N);
            i.1 = (i.1 + j.1).rem_euclid(M)
        });

        let avg_dist: usize = states
            .iter()
            .enumerate()
            .map(|(idx, pt1)| {
                let pt2 = &states[&states.len() - (idx + 1)];
                pt1.0.abs_diff(pt2.0) + pt1.1.abs_diff(pt2.1)
            })
            .sum();

        println!("{n_iter} {avg_dist}");

        if avg_dist < closest_dist {
            closest_dist = avg_dist;
            closest_iter = n_iter;
        }
    }
    closest_iter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day14.txt");
        let result = part1(&contents);
        assert_eq!(result, 233_709_840);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day14.txt");
        let result = part2(&contents);
        assert_eq!(result, 6_620);
    }
}
