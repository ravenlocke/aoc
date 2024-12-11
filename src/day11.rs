use crate::utils::SmallVec;

fn solve_for_n(n_iters: u8, content: &str) -> usize {
    let mut stack = SmallVec::<(u8, usize), 1_000>::default();

    let mut n = 0;
    for byte in content.bytes() {
        match byte {
            b'0'..b':' => {
                n *= 10;
                n += (byte - 48) as usize
            }
            _ => {
                stack.push((0, n));
                n = 0;
            }
        }
    }

    let mut total = 0;
    while stack.len() != 0 {
        let (iter, val) = stack.pop();
        if iter == n_iters - 1 {
            // If it splits in two, update total by 2.
            if val != 0 && val.ilog10() % 2 == 1 {
                total += 2;
            } else {
                total += 1;
            }
        } else {
            if val == 0 {
                stack.push((iter + 1, 1));
                continue;
            }
            let ilog = val.ilog10();
            if ilog % 2 == 0 {
                stack.push((iter + 1, val * 2024));
            } else {
                let divisor = 10usize.pow((ilog + 1) / 2);
                stack.push((iter + 1, val % divisor));
                stack.push((iter + 1, val / divisor));
            }
        }
    }
    total
}

pub fn part1(content: &str) -> usize {
    let mut stack = SmallVec::<(u8, usize), 1_000>::default();
    const N_ITERS: u8 = 25;

    let mut n = 0;
    for byte in content.bytes() {
        match byte {
            b'0'..b':' => {
                n *= 10;
                n += (byte - 48) as usize
            }
            _ => {
                stack.push((0, n));
                n = 0;
            }
        }
    }

    let mut total = 0;
    while stack.len() != 0 {
        let (iter, val) = stack.pop();
        if iter == N_ITERS - 1 {
            // If it splits in two, update total by 2.
            if val != 0 && val.ilog10() % 2 == 1 {
                total += 2;
            } else {
                total += 1;
            }
        } else {
            if val == 0 {
                stack.push((iter + 1, 1));
                continue;
            }
            let ilog = val.ilog10();
            if ilog % 2 == 0 {
                stack.push((iter + 1, val * 2024));
            } else {
                let divisor = 10usize.pow((ilog + 1) / 2);
                stack.push((iter + 1, val % divisor));
                stack.push((iter + 1, val / divisor));
            }
        }
    }
    total
}

pub fn part2(content: &str) -> usize {
    solve_for_n(75, content)
}
