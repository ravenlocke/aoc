use rustc_hash::FxHashMap;

struct NumberIter<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Iterator for NumberIter<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n = 0i64;
        let mut capture = false;

        for idx in self.pos..self.text.len() {
            let c = self.text.as_bytes()[idx];
            if c.is_ascii_digit() {
                capture = true;
                n *= 10;
                n += (c - 48) as i64;
            } else if capture {
                self.pos = idx + 1;
                return Some(n);
            }
        }

        // Check in case file has no newline at end of file.
        if capture {
            self.pos = self.text.len();
            return Some(n);
        }
        return None;
    }
}

#[inline(always)]
fn parse_input_part1(contents: &str) -> (Vec<i64>, Vec<i64>) {
    let mut list_a: Vec<i64> = Vec::new();
    let mut list_b: Vec<i64> = Vec::new();

    let mut populate_a = true;

    let number_iter = NumberIter {
        text: contents,
        pos: 0,
    };
    for number in number_iter {
        if populate_a {
            list_a.push(number);
        } else {
            list_b.push(number);
        }
        populate_a ^= true
    }

    (list_a, list_b)
}

#[inline(always)]
fn parse_input_part2(contents: &str) -> (Vec<i64>, FxHashMap<i64, i64>) {
    let mut list_a: Vec<i64> = Vec::new();
    let mut b_count_map = FxHashMap::default();

    let mut populate_a = true;

    let number_iter = NumberIter {
        text: contents,
        pos: 0,
    };
    for number in number_iter {
        if populate_a {
            list_a.push(number);
        } else {
            *b_count_map.entry(number).or_default() += 1;
        }
        populate_a ^= true
    }

    (list_a, b_count_map)
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

    #[test]
    fn test_part_one_example_solution() {
        let contents = include_str!("../inputs/day01_example.txt");
        let result = part1(&contents);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two_example_solution() {
        let contents = include_str!("../inputs/day01_example.txt");
        let result = part2(contents);
        assert_eq!(result, 31);
    }
}
