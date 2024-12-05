const SIZE: usize = 140;

fn lower_left_xmas_count(grid: &[&str]) -> u64 {
    let mut counter = 0;
    (0..SIZE - 3).for_each(|i| {
        (3..SIZE).for_each(|j| unsafe {
            let word = [
                *grid.get_unchecked(i).as_bytes().get_unchecked(j),
                *grid.get_unchecked(i + 1).as_bytes().get_unchecked(j - 1),
                *grid.get_unchecked(i + 2).as_bytes().get_unchecked(j - 2),
                *grid.get_unchecked(i + 3).as_bytes().get_unchecked(j - 3),
            ];
            if word == [b'X', b'M', b'A', b'S'] || word == [b'S', b'A', b'M', b'X'] {
                counter += 1
            }
        });
    });
    counter
}

fn left_right_xmas_count(grid: &[&str]) -> u64 {
    let mut counter = 0;
    (0..SIZE).for_each(|i| {
        (0..SIZE - 3).for_each(|j| unsafe {
            let word = [
                *grid.get_unchecked(i).as_bytes().get_unchecked(j),
                *grid.get_unchecked(i).as_bytes().get_unchecked(j + 1),
                *grid.get_unchecked(i).as_bytes().get_unchecked(j + 2),
                *grid.get_unchecked(i).as_bytes().get_unchecked(j + 3),
            ];
            if word == [b'X', b'M', b'A', b'S'] || word == [b'S', b'A', b'M', b'X'] {
                counter += 1
            }
        });
    });
    counter
}

fn up_down_xmas_count(grid: &[&str]) -> u64 {
    let mut counter = 0;
    (0..SIZE - 3).for_each(|i| {
        (0..SIZE).for_each(|j| unsafe {
            let word = [
                *grid.get_unchecked(i).as_bytes().get_unchecked(j),
                *grid.get_unchecked(i + 1).as_bytes().get_unchecked(j),
                *grid.get_unchecked(i + 2).as_bytes().get_unchecked(j),
                *grid.get_unchecked(i + 3).as_bytes().get_unchecked(j),
            ];
            if word == [b'X', b'M', b'A', b'S'] || word == [b'S', b'A', b'M', b'X'] {
                counter += 1
            }
        });
    });
    counter
}

fn lower_right_xmas_count(grid: &[&str]) -> u64 {
    let mut counter = 0;
    (0..SIZE - 3).for_each(|i| {
        (0..SIZE - 3).for_each(|j| unsafe {
            let word = [
                *grid.get_unchecked(i).as_bytes().get_unchecked(j),
                *grid.get_unchecked(i + 1).as_bytes().get_unchecked(j + 1),
                *grid.get_unchecked(i + 2).as_bytes().get_unchecked(j + 2),
                *grid.get_unchecked(i + 3).as_bytes().get_unchecked(j + 3),
            ];
            if word == [b'X', b'M', b'A', b'S'] || word == [b'S', b'A', b'M', b'X'] {
                counter += 1
            }
        });
    });
    counter
}

pub fn part1(content: &str) -> u64 {
    let matrix: Vec<&str> = content.lines().collect();

    lower_left_xmas_count(&matrix)
        + left_right_xmas_count(&matrix)
        + up_down_xmas_count(&matrix)
        + lower_right_xmas_count(&matrix)
}

pub fn part2(content: &str) -> u64 {
    let matrix: Vec<&str> = content.lines().collect();
    let mut counter = 0;

    (0..SIZE - 2).for_each(|i| {
        (0..SIZE - 2).for_each(|j| {
            if unsafe {
                (*matrix.get_unchecked(i).as_bytes().get_unchecked(j)
                    + *matrix.get_unchecked(i + 2).as_bytes().get_unchecked(j + 2))
                    | (*matrix.get_unchecked(i).as_bytes().get_unchecked(j + 2)
                        + *matrix.get_unchecked(i + 2).as_bytes().get_unchecked(j))
                    | *matrix.get_unchecked(i + 1).as_bytes().get_unchecked(j + 1)
                    == 225
            } {
                counter += 1
            }
        });
    });

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day04.txt");
        let result = part1(&contents);
        assert_eq!(result, 2_549);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day04.txt");
        let result = part2(contents);
        assert_eq!(result, 2_003);
    }
}
