macro_rules! check_and_increment_pt1 {
    ($ix:expr, $str:expr, $char:expr ) => {
        if $ix >= $str.len() || $str[$ix] != $char {
            return (0, $ix);
        }
        $ix += 1;
    };
}

macro_rules! check_and_increment_pt2 {
    ($ix:expr, $str:expr, $char:expr) => {
        if $ix >= $str.len() || $str[$ix] != $char {
            return (MulState::None, 0, $ix);
        }
        $ix += 1;
    };
}
fn parse_radix_10(input: &[u8]) -> u64 {
    let mut n = 0;
    for i in input {
        n *= 10;
        n += *i as u64
    }

    let correction = {
        match input.len() {
            1 => 48,
            2 => 528,
            3 => 5_328,
            _ => panic!("unexpected number length to correct for"),
        }
    };

    n -= correction;

    n
}

fn parse_mul(string: &[u8], mut idx: usize) -> (u64, usize) {
    let start = idx;
    while idx < string.len() && string[idx].is_ascii_digit() {
        idx += 1
    }
    let m = parse_radix_10(&string[start..idx]);

    check_and_increment_pt1!(idx, string, b',');

    let start = idx;
    while idx < string.len() && string[idx].is_ascii_digit() {
        idx += 1
    }

    let n = parse_radix_10(&string[start..idx]);
    check_and_increment_pt1!(idx, string, b')');

    (m * n, idx)
}

#[derive(Debug)]
enum MulState {
    Enable,
    Disable,
    None,
}

fn parse_do_dont_mul(string: &[u8], mut idx: usize) -> (MulState, u64, usize) {
    if idx >= string.len() {
        return (MulState::None, 0, idx);
    }

    match string[idx] {
        b'm' => {
            idx += 1;

            check_and_increment_pt2!(idx, string, b'u');
            check_and_increment_pt2!(idx, string, b'l');
            check_and_increment_pt2!(idx, string, b'(');

            let start = idx;
            while idx < string.len() && string[idx].is_ascii_digit() {
                idx += 1
            }
            let m = parse_radix_10(&string[start..idx]);

            check_and_increment_pt2!(idx, string, b',');

            let start = idx;
            while idx < string.len() && string[idx].is_ascii_digit() {
                idx += 1
            }
            let n = parse_radix_10(&string[start..idx]);

            check_and_increment_pt2!(idx, string, b')');
            (MulState::None, m * n, idx)
        }
        b'd' => {
            idx += 1;

            check_and_increment_pt2!(idx, string, b'o');

            match string[idx] {
                b'n' => {
                    idx += 1;
                    check_and_increment_pt2!(idx, string, b'\'');
                    check_and_increment_pt2!(idx, string, b't');
                    check_and_increment_pt2!(idx, string, b'(');
                    check_and_increment_pt2!(idx, string, b')');
                    (MulState::Disable, 0, idx)
                }
                b'(' => {
                    idx += 1;
                    check_and_increment_pt2!(idx, string, b')');
                    (MulState::Enable, 0, idx)
                }
                _ => (MulState::None, 0, idx),
            }
        }
        _ => (MulState::None, 0, idx + 1),
    }
}

pub fn part1(content: &str) -> u64 {
    let mut idx = 0;
    let mut result = 0;

    while idx + 4 < content.len() {
        if &content.as_bytes()[idx..idx + 4] != b"mul(" {
            idx += 1;
            continue;
        }
        let tmp;

        (tmp, idx) = parse_mul(content.as_bytes(), idx + 4);
        result += tmp;
    }
    result
}

pub fn part2(content: &str) -> u64 {
    let mut matching = true;
    let mut total = 0;
    let mut idx = 0;

    while idx < content.len() {
        if !matching && content.as_bytes()[idx] != b'd' {
            idx += 1;
            continue;
        }
        let change_update;
        let tmp;
        (change_update, tmp, idx) = parse_do_dont_mul(content.as_bytes(), idx);

        match change_update {
            MulState::Disable => matching = false,
            MulState::Enable => matching = true,
            _ => {}
        }

        if matching {
            total += tmp
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day03.txt");
        let result = part1(&contents);
        assert_eq!(result, 182_780_583);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day03.txt");
        let result = part2(contents);
        assert_eq!(result, 90_772_405);
    }
}
