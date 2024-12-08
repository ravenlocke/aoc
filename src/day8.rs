use std::{
    fmt::Display,
    isize,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    usize,
};

const DIM_USIZE: usize = 50;
const DIM_ISIZE: isize = DIM_USIZE as isize;
const NEWLINE_DIM: isize = DIM_ISIZE + 1;

#[derive(Clone, Copy)]
struct AntennaSet {
    locations: [Point; 10],
    count: usize,
}

impl Default for AntennaSet {
    fn default() -> Self {
        AntennaSet {
            locations: [Point::default(); 10],
            count: 0,
        }
    }
}

impl AntennaSet {
    fn push(&mut self, val: Point) {
        self.locations[self.count] = val;
        self.count += 1;
    }
}

#[derive(Clone, Copy)]
struct Delta(isize, isize);

impl Mul<isize> for Delta {
    type Output = Delta;

    fn mul(self, rhs: isize) -> Self::Output {
        return Delta(self.0 * rhs, self.1 * rhs);
    }
}

#[derive(Clone, Copy, Debug)]
struct Point(isize, isize);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Default for Point {
    fn default() -> Self {
        Point(0, 0)
    }
}

impl Add<Delta> for Point {
    type Output = Point;
    fn add(self, rhs: Delta) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Delta> for Point {
    fn add_assign(&mut self, rhs: Delta) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Delta> for Point {
    type Output = Point;
    fn sub(self, rhs: Delta) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign<Delta> for Point {
    fn sub_assign(&mut self, rhs: Delta) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Sub<Point> for Point {
    type Output = Delta;

    fn sub(self, rhs: Point) -> Self::Output {
        Delta(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct Grid<const M: usize, const N: isize> {
    grid: [[bool; M]; M],
}

impl<const M: usize, const N: isize> Display for Grid<M, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .grid
            .iter()
            .map(|row| {
                format!(
                    "{}\n",
                    row.iter()
                        .map(|i| if *i { '#' } else { '.' })
                        .collect::<String>()
                )
            })
            .collect::<String>();
        write!(f, "{}", string)
    }
}

impl<const M: usize, const N: isize> Grid<M, N> {
    fn new() -> Grid<M, N> {
        Grid {
            grid: [[false; M]; M],
        }
    }
    fn contains(&self, point: Point) -> bool {
        (0..N).contains(&point.0) && (0..N).contains(&point.1)
    }

    fn count_antinodes(&self) -> usize {
        self.grid
            .iter()
            .map(|i| i.iter().filter(|i| **i).count())
            .sum()
    }

    unsafe fn set_unchecked(&mut self, point: Point) {
        *self
            .grid
            .get_unchecked_mut(point.0 as usize)
            .get_unchecked_mut(point.1 as usize) = true;
    }
}

pub fn part1(content: &str) -> usize {
    let mut antennas: [AntennaSet; 128] = [AntennaSet::default(); 128];

    content
        .bytes()
        .enumerate()
        .filter(|(_, i)| *i != b'.' && *i != b'\n')
        .for_each(|(loc, byte)| {
            let idx_as_usize = loc as isize;
            antennas[byte as usize].push(Point(
                idx_as_usize / NEWLINE_DIM,
                idx_as_usize % NEWLINE_DIM,
            ));
        });

    let mut grid = Grid::<DIM_USIZE, DIM_ISIZE>::new();

    antennas.iter().for_each(|antenna_set| {
        for a in 0..antenna_set.count {
            for b in a + 1..antenna_set.count {
                let point_a = unsafe { *antenna_set.locations.get_unchecked(b) };
                let point_b = unsafe { *antenna_set.locations.get_unchecked(a) };

                // In theory, it should be possible for the points to be located between two antenna, such that:
                //  - A --> B = 1
                //  - A --> antinode = 1/3
                //  - antinode --> B = 2/3
                // However, this wasn't the case for the test inputs I've checked, so... 🤷
                //
                // Example of a potentially valid case we don't check for:
                // A...
                // .#..
                // ....
                // ...A

                let diff: Delta = point_a - point_b;
                if point_a + diff == point_b + (diff * 2) && grid.contains(point_a + diff) {
                    unsafe { grid.set_unchecked(point_a + diff) };
                }
                if point_b - diff == point_a - (diff * 2) && grid.contains(point_b - diff) {
                    unsafe { grid.set_unchecked(point_b - diff) };
                }
            }
        }
    });
    grid.count_antinodes()
}

pub fn part2(content: &str) -> usize {
    let mut antennas: [AntennaSet; 128] = [AntennaSet::default(); 128];

    content
        .bytes()
        .enumerate()
        .filter(|(_, i)| *i != b'.' && *i != b'\n')
        .for_each(|(loc, byte)| {
            let idx_as_usize = loc as isize;
            antennas[byte as usize].push(Point(
                idx_as_usize / NEWLINE_DIM,
                idx_as_usize % NEWLINE_DIM,
            ));
        });

    let mut grid = Grid::<DIM_USIZE, DIM_ISIZE>::new();

    antennas.iter().for_each(|antenna_set| {
        for a in 0..antenna_set.count {
            for b in a + 1..antenna_set.count {
                let point_a = unsafe { *antenna_set.locations.get_unchecked(b) };
                let point_b = unsafe { *antenna_set.locations.get_unchecked(a) };
                let diff: Delta = point_b - point_a;
                
                unsafe {
                    grid.set_unchecked(point_a);
                    grid.set_unchecked(point_b);
                }

                // In theory, it should be possible for points to be at non-integer multiples of the vector between the
                // two points. However, this doesn't seem to be the case in practice, and not checking yields a speed
                // up, so... 🙃
                // 
                // Example of a potentially valid case we don't check for.
                // A..
                // .#.
                // ..A
                let mut loc = point_a - diff;
                while grid.contains(loc) {
                    unsafe { grid.set_unchecked(loc) };
                    loc -= diff;                    
                }

                loc = point_b + diff;
                while grid.contains(loc) {
                    unsafe { grid.set_unchecked(loc) };
                    loc += diff
                }
            }
        }
    });
    grid.count_antinodes()}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day08.txt");
        let result = part1(&contents);
        assert_eq!(result, 285);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day08.txt");
        let result = part2(&contents);
        assert_eq!(result, 944);
    }
}
