use crate::utils::SmallVec;

const GRID_SIZE: usize = 50;
const GRIDWPT2: usize = GRID_SIZE * 2;
const INST_SIZE: usize = 20_000;

fn parse_content_pt1(
    content: &str,
) -> (
    [[u8; GRID_SIZE]; GRID_SIZE],
    [u8; INST_SIZE],
    (usize, usize),
) {
    let mut parse_idx = 0;
    let mut inst_idx = 0;
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
    let mut inst = [0; INST_SIZE];
    let (mut i, mut j) = (0, 0);
    let mut robot_pos = (0, 0);

    while parse_idx < content.len() {
        let byte = content.as_bytes()[parse_idx];
        match byte {
            b'#' | b'.' | b'O' => {
                grid[i][j] = byte;
                j += 1;
            }
            b'\n' => {
                i += 1;
                j = 0;
            }
            b'>' | b'<' | b'^' | b'v' => {
                inst[inst_idx] = byte;
                inst_idx += 1;
            }
            b'@' => {
                grid[i][j] = byte;
                (robot_pos.0, robot_pos.1) = (i, j);
                j += 1;
            }
            _ => {}
        }
        parse_idx += 1;
    }

    (grid, inst, robot_pos)
}

pub fn parse_content_pt2(
    content: &str,
) -> ([[u8; GRIDWPT2]; GRID_SIZE], [u8; INST_SIZE], (usize, usize)) {
    let mut parse_idx = 0;
    let mut inst_idx = 0;
    let mut grid = [[0; GRIDWPT2]; GRID_SIZE];
    let mut inst = [0; INST_SIZE];
    let (mut i, mut j) = (0, 0);
    let mut robot_pos = (0, 0);

    while parse_idx < content.len() {
        let byte = content.as_bytes()[parse_idx];
        match byte {
            b'#' | b'.' => {
                grid[i][j] = byte;
                j += 1;
                grid[i][j] = byte;
                j += 1;
            }
            b'O' => {
                grid[i][j] = b'[';
                j += 1;
                grid[i][j] = b']';
                j += 1;
            }
            b'\n' => {
                i += 1;
                j = 0;
            }
            b'>' | b'<' | b'^' | b'v' => {
                inst[inst_idx] = byte;
                inst_idx += 1;
            }
            b'@' => {
                grid[i][j] = byte;
                (robot_pos.0, robot_pos.1) = (i, j);
                j += 1;
                grid[i][j] = b'.';
                j += 1;
            }
            _ => {}
        }
        parse_idx += 1;
    }

    (grid, inst, robot_pos)
}

pub fn part1(content: &str) -> usize {
    let (mut grid, instructions, mut robot_pos) = parse_content_pt1(content);

    for instr in &instructions {
        match instr {
            b'<' => {
                let mut new_j = robot_pos.1;
                loop {
                    new_j -= 1;
                    if grid[robot_pos.0][new_j] == b'#' {
                        break;
                    }
                    if grid[robot_pos.0][new_j] == b'.' {
                        (new_j..robot_pos.1).for_each(|j| {
                            grid[robot_pos.0][j] = grid[robot_pos.0][j + 1];
                        });
                        grid[robot_pos.0][robot_pos.1] = b'.';
                        robot_pos.1 -= 1;
                        break;
                    }
                }
            }
            b'>' => {
                let mut new_j = robot_pos.1;
                loop {
                    new_j += 1;
                    if grid[robot_pos.0][new_j] == b'#' {
                        break;
                    }
                    if grid[robot_pos.0][new_j] == b'.' {
                        (robot_pos.1..new_j).rev().for_each(|j| {
                            grid[robot_pos.0][j + 1] = grid[robot_pos.0][j];
                        });
                        grid[robot_pos.0][robot_pos.1] = b'.';
                        robot_pos.1 += 1;
                        break;
                    }
                }
            }
            b'^' => {
                let mut new_i = robot_pos.0;
                loop {
                    new_i -= 1;
                    if grid[new_i][robot_pos.1] == b'#' {
                        break;
                    }
                    if grid[new_i][robot_pos.1] == b'.' {
                        (new_i..robot_pos.0).for_each(|i| {
                            grid[i][robot_pos.1] = grid[i + 1][robot_pos.1];
                        });
                        grid[robot_pos.0][robot_pos.1] = b'.';
                        robot_pos.0 -= 1;
                        break;
                    }
                }
            }
            b'v' => {
                let mut new_i = robot_pos.0;
                loop {
                    new_i += 1;
                    if grid[new_i][robot_pos.1] == b'#' {
                        break;
                    }
                    if grid[new_i][robot_pos.1] == b'.' {
                        (robot_pos.0..new_i).rev().for_each(|i| {
                            grid[i + 1][robot_pos.1] = grid[i][robot_pos.1];
                        });
                        grid[robot_pos.0][robot_pos.1] = b'.';
                        robot_pos.0 += 1;
                        break;
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    let mut total = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == b'O' {
                total += i * 100 + j
            }
        }
    }
    total
}

pub fn part2(content: &str) -> usize {
    let (mut grid, instructions, mut robot_pos) = parse_content_pt2(content);

    for instr in &instructions {
        match instr {
            b'<' => {
                let mut new_j = robot_pos.1;
                loop {
                    new_j -= 1;
                    if grid[robot_pos.0][new_j] == b'#' {
                        break;
                    }
                    if grid[robot_pos.0][new_j] == b'.' {
                        (new_j..robot_pos.1).for_each(|j| {
                            grid[robot_pos.0][j] = grid[robot_pos.0][j + 1];
                        });
                        grid[robot_pos.0][robot_pos.1] = b'.';
                        robot_pos.1 -= 1;
                        break;
                    }
                }
            }
            b'>' => {
                let mut new_j = robot_pos.1;
                loop {
                    new_j += 1;
                    if grid[robot_pos.0][new_j] == b'#' {
                        break;
                    }
                    if grid[robot_pos.0][new_j] == b'.' {
                        (robot_pos.1..new_j).rev().for_each(|j| {
                            grid[robot_pos.0][j + 1] = grid[robot_pos.0][j];
                        });
                        grid[robot_pos.0][robot_pos.1] = b'.';
                        robot_pos.1 += 1;
                        break;
                    }
                }
            }
            b'^' => {
                let mut vec = SmallVec::<(usize, usize, u8), 1_000>::default();
                let mut grid_cp = grid;
                grid_cp[robot_pos.0][robot_pos.1] = b'.';
                vec.push((robot_pos.0 - 1, robot_pos.1, b'@'));

                let mut use_grid_cp = true;
                while vec.len() != 0 {
                    let (i, j, new_val) = vec.pop();

                    match grid[i][j] {
                        // Pushing into a box
                        b'[' => {
                            vec.push((i - 1, j, b'['));
                            if new_val != b'[' {
                                vec.push((i - 1, j + 1, b']'));
                                grid_cp[i][j + 1] = b'.'
                            }
                            grid_cp[i][j] = new_val
                        }
                        b']' => {
                            vec.push((i - 1, j, b']'));
                            if new_val != b']' {
                                vec.push((i - 1, j - 1, b'['));
                                grid_cp[i][j - 1] = b'.'
                            }
                            grid_cp[i][j] = new_val
                        }

                        // Pushing into an empty space
                        b'.' => {
                            grid_cp[i][j] = new_val;
                        }

                        // Pushing into a wall
                        b'#' => {
                            use_grid_cp = false;
                            break;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }

                if use_grid_cp {
                    grid = grid_cp;
                    robot_pos.0 -= 1
                }
            }
            b'v' => {
                let mut vec = SmallVec::<(usize, usize, u8), 1_000>::default();
                let mut grid_cp = grid;
                grid_cp[robot_pos.0][robot_pos.1] = b'.';
                vec.push((robot_pos.0 + 1, robot_pos.1, b'@'));

                let mut use_grid_cp = true;
                while vec.len() != 0 {
                    let (i, j, new_val) = vec.pop();

                    match grid_cp[i][j] {
                        // Pushing into a box
                        b'[' => {
                            vec.push((i + 1, j, b'['));
                            if new_val != b'[' {
                                vec.push((i + 1, j + 1, b']'));
                                grid_cp[i][j + 1] = b'.'
                            }
                            grid_cp[i][j] = new_val
                        }
                        b']' => {
                            vec.push((i + 1, j, b']'));
                            if new_val != b']' {
                                vec.push((i + 1, j - 1, b'['));
                                grid_cp[i][j - 1] = b'.'
                            }
                            grid_cp[i][j] = new_val
                        }

                        // Pushing into an empty space
                        b'.' => {
                            grid_cp[i][j] = new_val;
                        }

                        // Pushing into a wall
                        b'#' => {
                            use_grid_cp = false;
                            break;
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }

                if use_grid_cp {
                    grid = grid_cp;
                    robot_pos.0 += 1
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    for row in grid {
        for col in row {
            print!("{}", col as char)
        }
        print!("\n")
    }

    let mut total = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == b'[' {
                total += i * 100 + j
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day15.txt");
        let result = part1(&contents);
        assert_eq!(result, 1_413_675);
    }

    // #[test]
    // fn test_part_two_solution() {
    //     let contents = include_str!("../inputs/day14.txt");
    //     let result = part2(&contents);
    //     assert_eq!(result, 6_620);
    // }
}
