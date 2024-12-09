const N: usize = 19_999;

fn parse_input(content: &str) -> ([u8; N], usize) {
    let mut result = [0; N];
    let mut count = 0;
    (0..N).for_each(|idx: usize| {
        let val = content.as_bytes()[idx];
        result[idx] = val - 48;
        if idx % 2 == 0 {
            count += (result[idx]) as usize;
        }
    });
    (result, count)
}

pub fn part1(content: &str) -> usize {
    let (mut input, count) = parse_input(content);

    let mut fwd_idx = 0;
    let mut rev_idx = if input.len() % 2 == 1 {
        input.len() - 1
    } else {
        input.len() - 2
    };
    let mut counter = 0;
    let mut total = 0;

    loop {
        if fwd_idx % 2 == 0 {
            total += (fwd_idx / 2) * counter;
            input[fwd_idx] -= 1;
            while input[fwd_idx] == 0 {
                fwd_idx += 1;
            }
        } else {
            while input[rev_idx] == 0 {
                rev_idx -= 2;
            }
            total += (rev_idx / 2) * counter;
            input[rev_idx] -= 1;
            input[fwd_idx] -= 1;

            while input[fwd_idx] == 0 && counter != count {
                fwd_idx += 1;
            }
        }

        counter += 1;
        if counter == count {
            break;
        }
    }

    total
}

pub fn part2(content: &str) -> usize {
    let (mut input, _) = parse_input(content);
    let original_input = input;

    let mut arr = [N + 1; 10];
    let mut counter = 0;
    let mut total = 0;

    (0..N).for_each(|i| {
        if i % 2 == 0 {
            if input[i] == 0 {
                counter += original_input[i] as usize
            } else {
                (0..input[i]).for_each(|_| {
                    total += counter * (i / 2) as usize;
                    counter += 1
                });
            }
        } else {
            let mut capacity = input[i];
            let mut capacity_usize = capacity as usize;

            // While we have capcity to fill and haven't finished checking positions that may
            // have the capacity to move here.
            while capacity != 0 && arr[capacity_usize] > i + 2 {
                // Decrement
                arr[capacity_usize] -= 2;
                let capacity_move = &mut input[arr[capacity_usize]];
                // If that position can fill some or all of the capacity.
                if *capacity_move <= capacity {
                    // Update total and counter for it
                    (0..*capacity_move).for_each(|_| {
                        total += counter * (arr[capacity_usize] / 2) as usize;
                        counter += 1
                    });
                    // Mark it as moved and update capacity for the space being filled.
                    capacity -= *capacity_move;
                    *capacity_move = 0;
                    capacity_usize = capacity as usize;
                }
            }
            counter += capacity_usize
        }
    });

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day09.txt");
        let result = part1(&contents);
        assert_eq!(result, 6_390_180_901_651);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day09.txt");
        let result = part2(&contents);
        assert_eq!(result, 6_412_390_114_238);
    }
}
