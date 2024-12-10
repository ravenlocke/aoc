// TODO: Write to run in reverse (start from the target and iterate backwards through the list).
// TODO: Write to not use recursion (e.g., have two stacks, pop from one to the other).

struct SmallVec<T, const M: usize> {
    data: [T; M],
    counter: usize,
}

impl<T: Default + Copy, const M: usize> Default for SmallVec<T, M> {
    fn default() -> Self {
        SmallVec {
            data: [T::default(); M],
            counter: 0,
        }
    }
}

impl<T, const M: usize> SmallVec<T, M> {
    fn push(&mut self, item: T) {
        self.data[self.counter] = item;
        self.counter += 1;
    }
}

impl <'a, T, const M: usize> IntoIterator for &'a SmallVec<T, M> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data[0..self.counter].iter()
    }
}


struct ParsedRow {
    target: usize,
    numbers: SmallVec<usize, 20>,
}

impl ParsedRow {
    fn maybe_possible_pt1(&self) -> bool {
        self.numbers.data[1..self.numbers.counter]
            .iter()
            .fold(self.numbers.data[0], |sum, i| usize::max(sum + i, sum * i))
            >= self.target
    }
}

struct LineParser<'a> {
    content: &'a str,
    idx: usize,
}

impl LineParser<'_> {
    fn new(content: &str) -> LineParser {
        LineParser { content, idx: 0 }
    }
}

impl Iterator for LineParser<'_> {
    type Item = ParsedRow;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n = 0;
        let mut target = 0;
        let mut v = SmallVec::<usize, 20>::default();

        while self.idx < self.content.len() {
            let byte = unsafe { *self.content.as_bytes().get_unchecked(self.idx) };
            match byte {
                b'0'..b':' => {
                    n *= 10;
                    n += (byte - 48) as usize;
                    self.idx += 1
                }
                b':' => {
                    target = n;
                    n = 0;
                    self.idx += 2;
                }
                b' ' => {
                    v.push(n);
                    n = 0;
                    self.idx += 1
                }
                b'\n' => {
                    v.push(n);
                    self.idx += 1;
                    return Some(ParsedRow { target, numbers: v });
                }
                _ => self.idx += 1,
            }
        }
        None
    }
}

fn concatenate(a: usize, b: usize) -> usize {
    10usize.pow(b.ilog10() + 1) * a + b
}

unsafe fn is_valid_pt1(target: usize, current: usize, numbers: &[usize]) -> bool {
    if numbers.len() == 1 {
        return current + numbers.get_unchecked(0) == target
            || current * numbers.get_unchecked(0) == target;
    } else {
        return is_valid_pt1(target, current + numbers.get_unchecked(0), &numbers[1..])
            || is_valid_pt1(target, current * numbers.get_unchecked(0), &numbers[1..]);
    }
}

unsafe fn is_valid_pt2(target: usize, current: usize, numbers: &[usize]) -> bool {
    let add = current + numbers.get_unchecked(0);
    let mul = current * numbers.get_unchecked(0);
    let concat = concatenate(current, *numbers.get_unchecked(0));

    if numbers.len() == 1 {
        return add == target || mul == target || concat == target;
    } else {
        return (add <= target && is_valid_pt2(target, add, &numbers[1..]))
            || (mul <= target && is_valid_pt2(target, mul, &numbers[1..]))
            || (concat <= target && is_valid_pt2(target, concat, &numbers[1..]));
    }
}

pub fn part1(content: &str) -> usize {
    let mut total = 0;

    for row in LineParser::new(content) {
        if !row.maybe_possible_pt1() {
            continue;
        }
        if unsafe { is_valid_pt1(row.target, 0, &row.numbers.data[0..row.numbers.counter]) } {
            total += row.target
        }
    }

    total
}

pub fn part2(content: &str) -> usize {
    let rows = LineParser::new(content).collect::<Vec<_>>();

    rows.iter()
        .map(|row| {
            if unsafe { is_valid_pt2(row.target, 0, &row.numbers.data[0..row.numbers.counter]) } {
                row.target
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day07.txt");
        let result = part1(&contents);
        assert_eq!(result, 20_281_182_715_321);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day07.txt");
        let result = part2(&contents);
        assert_eq!(result, 159_490_400_628_354);
    }
}
