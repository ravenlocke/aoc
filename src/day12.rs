use crate::utils::SmallVec;
use crate::day12_luts::CORNER_LUT;

const N: usize = 140;
const M: usize = N + 2;

fn expand_pt2(i: usize, j: usize, grid: &[[u8; M]; M], visited: &mut [[bool; M]; M]) -> usize {
    // Sum of internal angles = (n - 2) * 180
    let mut area = 0;
    let mut n_sizes = 0;
    let mut vec = SmallVec::<(usize, usize), 1_000>::default();
    vec.push((i, j));
    visited[i][j] = true;

    while vec.len() != 0 {
        let mut borders = [false, false, false, false];

        area += 1;

        let (i, j) = vec.pop();

        if grid[i][j] != grid[i][j - 1] {
            // Borders a different field, update perimiter.
            borders[0] = true;
        } else {
            if !visited[i][j - 1] {
                // Mark it to visit and add it to places to check.
                visited[i][j - 1] = true;
                vec.push((i, j - 1));
            }
        }

        if grid[i][j] != grid[i][j + 1] {
            borders[2] = true;
        } else {
            if !visited[i][j + 1] {
                visited[i][j + 1] = true;
                vec.push((i, j + 1));
            }
        }

        if grid[i][j] != grid[i - 1][j] {
            borders[1] = true
        } else {
            if !visited[i - 1][j] {
                visited[i - 1][j] = true;
                vec.push((i - 1, j));
            }
        }

        if grid[i][j] != grid[i + 1][j] {
            borders[3] = true
        } else {
            if !visited[i + 1][j] {
                visited[i + 1][j] = true;
                vec.push((i + 1, j));
            }
        }

        let val = grid[i][j];
        let n = 0usize
            | (128 * (grid[i - 1][j - 1] == val) as usize)
            | (64 * (grid[i - 1][j] == val) as usize)
            | (32 * (grid[i - 1][j + 1] == val) as usize)
            | (16 * (grid[i][j - 1] == val) as usize)
            | (8 * (grid[i][j + 1] == val) as usize)
            | (4 * (grid[i + 1][j - 1] == val) as usize)
            | (2 * (grid[i + 1][j] == val) as usize)
            | (1 * (grid[i + 1][j + 1] == val) as usize);

        n_sizes += CORNER_LUT[n]
    }

    return area * n_sizes;
}

fn expand(i: usize, j: usize, grid: &[[u8; M]; M], visited: &mut [[bool; M]; M]) -> usize {
    let mut area = 0;
    let mut perimiter = 0;
    let mut vec = SmallVec::<(usize, usize), 1_000>::default();
    vec.push((i, j));
    visited[i][j] = true;

    while vec.len() != 0 {
        area += 1;

        let (i, j) = vec.pop();

        if grid[i][j] != grid[i][j - 1] {
            perimiter += 1;
        } else {
            if !visited[i][j - 1] {
                visited[i][j - 1] = true;
                vec.push((i, j - 1));
            }
        }

        if grid[i][j] != grid[i][j + 1] {
            perimiter += 1;
        } else {
            if !visited[i][j + 1] {
                visited[i][j + 1] = true;
                vec.push((i, j + 1));
            }
        }

        if grid[i][j] != grid[i - 1][j] {
            perimiter += 1;
        } else {
            if !visited[i - 1][j] {
                visited[i - 1][j] = true;
                vec.push((i - 1, j));
            }
        }

        if grid[i][j] != grid[i + 1][j] {
            perimiter += 1;
        } else {
            if !visited[i + 1][j] {
                visited[i + 1][j] = true;
                vec.push((i + 1, j));
            }
        }
    }

    return area * perimiter;
}

pub fn part1(content: &str) -> usize {
    let mut grid: [[u8; M]; M] = [[b'.'; M]; M];

    let (mut i, mut j) = (1, 1);
    content.bytes().for_each(|byte| match byte {
        b'\n' => {
            i += 1;
            j = 1;
        }
        _ => {
            grid[i][j] = byte;
            j += 1;
        }
    });

    let mut visited: [[bool; M]; M] = [[false; M]; M];
    let mut total = 0;

    (1..N + 1).for_each(|row_idx| {
        (1..N + 1).for_each(|col_idx| {
            if !visited[row_idx][col_idx] {
                total += expand(row_idx, col_idx, &grid, &mut visited)
            }
        });
    });

    total
}

pub fn part2(content: &str) -> usize {
    let mut grid: [[u8; M]; M] = [[b'0'; M]; M];

    let (mut i, mut j) = (1, 1);
    content.bytes().for_each(|byte| match byte {
        b'\n' => {
            i += 1;
            j = 1;
        }
        _ => {
            grid[i][j] = byte;
            j += 1;
        }
    });

    let mut visited: [[bool; M]; M] = [[false; M]; M];
    let mut total = 0;

    (1..N + 1).for_each(|row_idx| {
        (1..N + 1).for_each(|col_idx| {
            if !visited[row_idx][col_idx] {
                total += expand_pt2(row_idx, col_idx, &grid, &mut visited)
            }
        });
    });

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day12.txt");
        let result = part1(&contents);
        assert_eq!(result, 1_375_476);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day12.txt");
        let result = part2(&contents);
        assert_eq!(result, 821_372);
    }
}
