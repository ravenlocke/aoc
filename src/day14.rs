const M: isize = 103;
const N: isize = 101;

const MID_M: isize = M / 2;
const MID_N: isize = N / 2;

const N_POSITIONS: f64 = 500.;
const MAX_STATES: isize = M * N;

fn calculate_variance(i: impl Iterator<Item = isize>) -> f64 {
    let mut s = 0f64;
    let mut s2 = 0f64;

    i.into_iter().for_each(|i| {
        s += i as f64;
        s2 += (i as f64) * (i as f64);
    });

    (s2 - (s * s) / N_POSITIONS) / N_POSITIONS
}

pub fn part1(content: &str) -> isize {
    const N_ITERS: isize = 100;
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
                (digits[0] + N_ITERS * digits[2]).rem_euclid(N),
                (digits[1] + N_ITERS * digits[3]).rem_euclid(M),
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

    let mut states = Vec::with_capacity(N_POSITIONS as usize);
    let mut deltas = Vec::with_capacity(N_POSITIONS as usize);

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

    let mut var_y_min = f64::MAX;
    let mut var_y_min_val = 0isize;

    let mut var_x_min = f64::MAX;
    let mut var_x_min_val = 0isize;

    for n_iter in 0..103 {
        (&mut states).iter_mut().zip(&deltas).for_each(|(i, j)| {
            i.0 = (i.0 + j.0).rem_euclid(N);
            i.1 = (i.1 + j.1).rem_euclid(M)
        });

        let var_x = calculate_variance(states.iter().map(|i| i.0));
        if var_x < var_x_min {
            var_x_min = var_x;
            var_x_min_val = n_iter
        }

        let var_y = calculate_variance(states.iter().map(|i| i.1));
        if var_y < var_y_min {
            var_y_min = var_y;
            var_y_min_val = n_iter
        }
    }
    for i in 0..MAX_STATES {
        if i % 101 == var_x_min_val && i % 103 == var_y_min_val {
            return i + 1;
        }
    }

    panic!("We were doomed from the very start.")
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
