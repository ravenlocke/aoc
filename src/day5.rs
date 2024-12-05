use std::cmp::Ordering;

fn parse_two_digit_radix_10(input: &[u8]) -> usize {
    let mut n = 0;
    for i in input {
        n *= 10;
        n += *i as usize
    }
    n -= 528;

    n
}


struct LineParser <'a> {
    idx: usize,
    content: &'a str
}

impl LineParser <'_> {
    fn new(content: &str) -> LineParser {
        LineParser{content, idx: 0}
    }

    fn parse_next(&mut self, vec: &mut Vec<usize>) -> bool {
        while self.idx + 2 < self.content.len() {
            let x = parse_two_digit_radix_10( &self.content.as_bytes()[self.idx..self.idx+2] );
            vec.push(x);
            
            if self.content.as_bytes()[self.idx + 2] == b'\n' {
                self.idx += 3;
                return true
            }

            self.idx += 3;
        }

        false
    }
}

fn is_ordered(vec: &Vec<usize>, rels: &[[Ordering; 128]; 128]) -> bool {
    (0..vec.len()-1).all(|i| rels[vec[i]][vec[i+1]] == Ordering::Less)
}

pub fn part1(content: &str) -> u64 {
    let mut rels = [[Ordering::Greater; 128]; 128];
    for i in 0..1176 {
        let idx = i * 6;
        let a = parse_two_digit_radix_10(&content.as_bytes()[idx..idx + 2]);
        let b = parse_two_digit_radix_10(&content.as_bytes()[idx + 3..idx + 5]);

        rels[a][b] = Ordering::Less;
    }

    let mut lp = LineParser::new(&content[1_176 * 6 + 1 ..]);
    let mut vec = Vec::<usize>::with_capacity(23);
    let mut total = 0;
    while lp.parse_next(&mut vec) {
        if is_ordered(&vec, &rels) {
            total += vec[vec.len() / 2]
        }
        vec.clear();
    }
    total as u64
}

pub fn part2(content: &str) -> u64 {
    let mut rels = [[Ordering::Greater; 128]; 128];
    for i in 0..1176 {
        let idx = i * 6;
        let a = parse_two_digit_radix_10(&content.as_bytes()[idx..idx + 2]);
        let b = parse_two_digit_radix_10(&content.as_bytes()[idx + 3..idx + 5]);

        rels[a][b] = Ordering::Less;
    }

    let mut lp = LineParser::new(&content[1_176 * 6 + 1 ..]);
    let mut vec = Vec::<usize>::with_capacity(23);
    let mut total = 0;

    while lp.parse_next(&mut vec) {
        if !is_ordered(&vec, &rels) {
            vec.sort_unstable_by(|a, b| rels[*a][*b]);
            total += vec[vec.len() / 2]
        }
        vec.clear();
    }
    
    total as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day05.txt");
        let result = part1(&contents);
        assert_eq!(result, 7_024);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day05.txt");
        let result = part2(&contents);
        assert_eq!(result, 4_151);
    }
}
