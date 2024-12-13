use std::{arch::asm, f32::consts::LN_10, isize};

fn bytes_to_isize(bytes: &[u8]) -> isize {
    let mut n = 0isize;
    for byte in bytes {
        n *= 10;
        n += (byte - b'0') as isize
    }
    n
}

#[derive(Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn new(a: (isize, isize), b: (isize, isize), prize: (isize, isize)) -> Machine {
        Machine { a, b, prize }
    }

    fn solve(&mut self) -> isize {
        let mut b_num = 0;

        unsafe {
            asm!(
                "imul r9",
                "mov {b_num:r}, rax",
                "mov rax, r10",
                "imul r11",
                "sub {b_num}, rax",

                b_num = inout(reg) b_num,
                in("rax") self.a.1,
                in("r9") self.prize.0,
                in("r10") self.a.0,
                in("r11") self.prize.1,
            )
        }

        let b_den = self.b.0 * self.a.1 - self.a.0 * self.b.1;

        if b_num % b_den != 0 {
            return 0;
        }
        let b = b_num / b_den;

        if (self.prize.0 - b * self.b.0) % self.a.0 != 0 {
            return 0;
        }

        let a = (self.prize.0 - b * self.b.0) / self.a.0;

        return 3 * a + b;
    }
}

struct MachineParser<'a> {
    content: &'a str,
    completed: bool,
}

impl MachineParser<'_> {
    fn new(content: &str) -> MachineParser {
        MachineParser {
            content,
            completed: false,
        }
    }
}

impl Iterator for MachineParser<'_> {
    type Item = Machine;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }
        let bytes = &self.content.as_bytes();
        let a = (
            bytes_to_isize(&bytes[12..14]),
            bytes_to_isize(&bytes[18..20]),
        );
        let b = (
            bytes_to_isize(&bytes[33..35]),
            bytes_to_isize(&bytes[39..41]),
        );

        self.content = &self.content[51..];

        // Parse the X location of the prize
        let mut i = 0;
        while (b'0'..b':').contains(&self.content.as_bytes()[i]) {
            i += 1;
        }
        let target_x = bytes_to_isize(&self.content.as_bytes()[..i]);
        self.content = &self.content[i + 4..];

        // Parse the Y location of the prize
        i = 0;
        while (b'0'..b':').contains(&self.content.as_bytes()[i]) {
            i += 1;
        }
        let target_y = bytes_to_isize(&self.content.as_bytes()[..i]);

        // Prepare for the next loop
        while i < self.content.as_bytes().len() && self.content.as_bytes()[i] == b'\n' {
            i += 1;
        }
        if i == self.content.as_bytes().len() {
            self.completed = true
        } else {
            self.content = &self.content[i..];
        }

        let machine = Machine::new(a, b, (target_x, target_y));
        Some(machine)
    }
}

pub fn part1(content: &str) -> isize {
    MachineParser::new(content).map(|mut i| i.solve()).sum()
}

pub fn part2(content: &str) -> isize {
    MachineParser::new(content)
        .map(|mut machine| {
            machine.prize = (
                machine.prize.0 + 10000000000000,
                machine.prize.1 + 10000000000000,
            );
            machine.solve()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day13.txt");
        let result = part1(&contents);
        assert_eq!(result, 40_069);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day13.txt");
        let result = part2(&contents);
        assert_eq!(result, 71_493_195_288_102);
    }
}
