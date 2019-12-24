const BUG: char = '#';
const COLS: u32 = 5;
const ROWS: u32 = 5;
const SIZE: u32 = 25;
const INPUT_PATH: &str = "inputs/day24.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait Eris {
    fn from_str(input: &str) -> Self;
    fn next(self) -> Self;
    fn count_bugs(&self) -> u32;
}

impl Eris for u32 {
    fn from_str(input: &str) -> Self {
        input
            .lines()
            .flat_map(str::chars)
            .enumerate()
            .fold(0, |acc, (idx, tile)| {
                if tile == BUG {
                    acc | (1 << (idx as u32))
                } else {
                    acc
                }
            })
    }

    fn next(self) -> Self {
        (0..SIZE)
            .filter(|&idx| {
                let (x, y) = (idx % COLS, idx / ROWS);
                let mut neighbors = 0;

                if x != 0 {
                    neighbors |= 1 << (idx - 1);
                }

                if x != (COLS - 1) {
                    neighbors |= 1 << (idx + 1);
                }

                if y != 0 {
                    neighbors |= 1 << (idx - ROWS);
                }

                if y != (ROWS - 1) {
                    neighbors |= 1 << (idx + ROWS);
                }

                let count = (neighbors & self).count_ones();
                let bug = 1 << idx;

                count == 1 || (count == 2 && (self & bug) == 0)
            })
            .fold(0, |acc, idx| acc | (1 << idx))
    }

    fn count_bugs(&self) -> u32 {
        self.count_ones()
    }
}

impl Eris for Vec<u32> {
    fn from_str(input: &str) -> Self {
        vec![u32::from_str(input)]
    }

    fn next(self) -> Self {
        let mut levels = Vec::with_capacity(self.len() + 4);
        levels.push(0);
        levels.push(0);
        levels.extend(self);
        levels.push(0);
        levels.push(0);

        levels
            .windows(3)
            .map(|window| {
                let outer = window[0];
                let level = window[1];
                let inner = window[2];

                (0..SIZE)
                    .filter(|&idx| {
                        if idx == 12 {
                            return false;
                        }

                        let (x, y) = (idx % COLS, idx / ROWS);
                        let mut neighbors = 0;

                        if x != 0 {
                            neighbors |= 1 << (idx - 1);
                        }

                        if x != (COLS - 1) {
                            neighbors |= 1 << (idx + 1);
                        }

                        if y != 0 {
                            neighbors |= 1 << (idx - ROWS);
                        }

                        if y != (ROWS - 1) {
                            neighbors |= 1 << (idx + ROWS);
                        }

                        let outer_neighbors = match idx {
                            0 => 0b00000_00000_00010_00100_00000,
                            1..=3 => 0b00000_00000_00000_00100_00000,
                            4 => 0b00000_00000_01000_00100_00000,
                            5 | 10 | 15 => 0b00000_00000_00010_00000_00000,
                            9 | 14 | 19 => 0b00000_00000_01000_00000_00000,
                            20 => 0b00000_00100_00010_00000_00000,
                            21..=23 => 0b00000_00100_00000_00000_00000,
                            24 => 0b00000_00100_01000_00000_00000,
                            _ => 0,
                        };

                        let inner_neighbours = match idx {
                            7 => 0b00000_00000_00000_00000_11111,
                            11 => 0b00001_00001_00001_00001_00001,
                            13 => 0b10000_10000_10000_10000_10000,
                            17 => 0b11111_00000_00000_00000_00000,
                            _ => 0,
                        };

                        let count = (outer & outer_neighbors).count_ones()
                            + (level & neighbors).count_ones()
                            + (inner & inner_neighbours).count_ones();

                        let bug = 1 << idx;

                        count == 1 || (count == 2 && (level & bug) == 0)
                    })
                    .fold(0, |acc, idx| acc | (1 << idx))
            })
            .collect()
    }

    fn count_bugs(&self) -> u32 {
        self.iter().map(|level| level.count_bugs()).sum()
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    part1(&input);
    part2(&input);

    Ok(())
}

/// What is the biodiversity rating for the first layout that appears twice?
fn part1(input: &str) {
    let mut eris: u32 = Eris::from_str(input);
    let mut ratings = std::collections::HashSet::new();
    while ratings.insert(eris) {
        eris = eris.next();
    }

    println!("Part 1: {}", eris);
}

/// Starting with your scan, how many bugs are present after 200 minutes?
fn part2(input: &str) {
    let mut eris = Vec::from_str(input);
    for _ in 0..200 {
        eris = eris.next();
    }

    let part2 = eris.count_bugs();
    println!("Part 2: {}", part2);
}
