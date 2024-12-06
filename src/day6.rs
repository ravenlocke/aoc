use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rustc_hash::FxHashSet;

const GRID_DIM: usize = 130;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    NORTH = 0,
    SOUTH = 1,
    EAST = 2,
    WEST = 3,
}

struct LoopChecker<'a> {
    position: (usize, usize),
    visited: FxHashSet<usize>,
    direction: Direction,
    grid: &'a [[bool; GRID_DIM]],
}

fn pos_and_direction_to_usize(position: (usize, usize), direction: Direction) -> usize {
    0 | (position.0 << 11) | (position.1 << 2) | direction as usize
}

impl LoopChecker<'_> {
    fn has_loop(&mut self) -> bool {
        loop {
            match self.direction {
                Direction::NORTH => {
                    while self.position.0 > 0 && self.grid[self.position.0 - 1][self.position.1] {
                        self.position.0 -= 1;
                        if !self
                            .visited
                            .insert(pos_and_direction_to_usize(self.position, self.direction))
                        {
                            return true;
                        }
                    }
                    if self.position.0 == 0 {
                        return false;
                    }
                }
                Direction::SOUTH => {
                    while self.position.0 < GRID_DIM - 1
                        && self.grid[self.position.0 + 1][self.position.1]
                    {
                        self.position.0 += 1;
                        if !self
                            .visited
                            .insert(pos_and_direction_to_usize(self.position, self.direction))
                        {
                            return true;
                        }
                    }

                    if self.position.0 == GRID_DIM - 1 {
                        return false;
                    }
                }
                Direction::EAST => {
                    while self.position.1 < GRID_DIM - 1
                        && self.grid[self.position.0][self.position.1 + 1]
                    {
                        self.position.1 += 1;
                        if !self
                            .visited
                            .insert(pos_and_direction_to_usize(self.position, self.direction))
                        {
                            return true;
                        }
                    }
                    if self.position.1 == GRID_DIM - 1 {
                        return false;
                    }
                }
                Direction::WEST => {
                    while self.position.1 > 0 && self.grid[self.position.0][self.position.1 - 1] {
                        self.position.1 -= 1;
                        if !self
                            .visited
                            .insert(pos_and_direction_to_usize(self.position, self.direction))
                        {
                            return true;
                        }
                    }
                    if self.position.1 == 0 {
                        return false;
                    }
                }
            };

            self.direction = match self.direction {
                Direction::EAST => Direction::SOUTH,
                Direction::NORTH => Direction::EAST,
                Direction::WEST => Direction::NORTH,
                Direction::SOUTH => Direction::WEST,
            }
        }
    }
}

struct MovementTracker<'a> {
    position: (usize, usize),
    visited: Vec<(usize, usize)>,
    direction: Direction,
    grid: &'a [[bool; GRID_DIM]],
}

impl MovementTracker<'_> {
    fn move_untl_end(&mut self) {
        while self.move_in_direction() {
            self.direction = match self.direction {
                Direction::EAST => Direction::SOUTH,
                Direction::NORTH => Direction::EAST,
                Direction::WEST => Direction::NORTH,
                Direction::SOUTH => Direction::WEST,
            }
        }
    }

    fn spaces_visited(&mut self) -> FxHashSet<&(usize, usize)> {
        self.visited.iter().collect::<FxHashSet<_>>()
    }

    fn move_in_direction(&mut self) -> bool {
        match self.direction {
            Direction::NORTH => {
                while self.position.0 > 0 && self.grid[self.position.0 - 1][self.position.1] {
                    self.position.0 -= 1;
                    self.visited.push(self.position);
                }
                if self.position.0 == 0 {
                    return false;
                }
                true
            }
            Direction::SOUTH => {
                while self.position.0 < GRID_DIM - 1
                    && self.grid[self.position.0 + 1][self.position.1]
                {
                    self.position.0 += 1;
                    self.visited.push(self.position);
                }

                if self.position.0 == GRID_DIM - 1 {
                    return false;
                }
                return true;
            }
            Direction::EAST => {
                while self.position.1 < GRID_DIM - 1
                    && self.grid[self.position.0][self.position.1 + 1]
                {
                    self.position.1 += 1;
                    self.visited.push(self.position);
                }
                if self.position.1 == GRID_DIM - 1 {
                    return false;
                }
                true
            }
            Direction::WEST => {
                while self.position.1 > 0 && self.grid[self.position.0][self.position.1 - 1] {
                    self.position.1 -= 1;
                    self.visited.push(self.position);
                }
                if self.position.1 == 0 {
                    return false;
                }
                true
            }
        }
    }
}

fn parse_content(content: &str) -> ((usize, usize), [[bool; GRID_DIM]; GRID_DIM]) {
    let mut arr = [[true; GRID_DIM]; GRID_DIM];
    let mut guard_start = (usize::MAX, usize::MAX);
    let mut i = 0;
    let mut j = 0;

    for byte in content.bytes() {
        match byte {
            b'.' => j += 1,
            b'\n' => {
                j = 0;
                i += 1
            }
            b'#' => {
                arr[i][j] = false;
                j += 1
            }
            b'^' => {
                guard_start = (i, j);
                j += 1
            }
            _ => unreachable!(),
        }
    }

    (guard_start, arr)
}

pub fn part1(content: &str) -> u64 {
    let (guard_start, grid) = parse_content(content);

    let mut mt = MovementTracker {
        position: guard_start,
        visited: Vec::with_capacity(10_000),
        direction: Direction::NORTH,
        grid: &grid,
    };
    mt.visited.push(mt.position);

    mt.move_untl_end();
    mt.spaces_visited().len() as u64
}

pub fn part2(content: &str) -> usize {
    let (guard_start, grid) = parse_content(content);

    let mut mt = MovementTracker {
        position: guard_start,
        visited: Vec::with_capacity(10_000),
        direction: Direction::NORTH,
        grid: &grid,
    };

    mt.move_untl_end();

    mt.spaces_visited()
        .par_iter()
        .filter(|space| {
            let mut mod_grid = grid;
            mod_grid[space.0][space.1] = false;
            let mut loop_check = LoopChecker {
                grid: &mod_grid,
                position: guard_start,
                visited: FxHashSet::default(),
                direction: Direction::NORTH,
            };
            loop_check.has_loop()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day06.txt");
        let result = part1(&contents);
        assert_eq!(result, 4_977);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day06.txt");
        let result = part2(&contents);
        assert_eq!(result, 1_729);
    }
}
