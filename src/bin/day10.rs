use num::integer::Integer;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashSet};

const ASTEROID: char = '#';
const INPUT_PATH: &str = "inputs/day10.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn direction(self, other: Self) -> Option<Direction> {
        if self == other {
            None
        } else {
            Some(Direction::new(
                other.x as isize - self.x as isize,
                other.y as isize - self.y as isize,
            ))
        }
    }

    /// By reducing the direction with their GCD, we get the equivalent of a
    /// unit vector: (1, 2) and (2, 4) are now equivalent.
    fn reduced_direction(self, other: Self) -> Option<Direction> {
        self.direction(other).map(Direction::reduce)
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;

    fn add(self, d: Direction) -> Self {
        Position::new(
            (self.x as isize + d.x) as usize,
            (self.y as isize + d.y) as usize,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    fn new(x: isize, y: isize) -> Self {
        Direction { x, y }
    }

    fn reduce(self) -> Self {
        let gcd = self.x.gcd(&self.y);
        Direction::new(self.x / gcd, self.y / gcd)
    }

    fn squared_distance(self) -> isize {
        self.x.pow(2) + self.y.pow(2)
    }

    fn to_radians(self) -> f64 {
        (self.x as f64).atan2(self.y as f64)
    }
}

/// Implement clockwise ordering.
impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_radians()
            .partial_cmp(&other.to_radians())
            .map(|ord| {
                ord.reverse()
                    .then(self.squared_distance().cmp(&other.squared_distance()))
            })
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Your job is to figure out which asteroid would be the best place to build a
/// new monitoring station. A monitoring station can detect any asteroid to
/// which it has direct line of sight - that is, there cannot be another
/// asteroid exactly between them. This line of sight can be at any angle, not
/// just lines aligned to the grid or diagonally. The best location is the
/// asteroid that can detect the largest number of other asteroids.
fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let asteroids: Vec<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, ch)| {
                if ch == ASTEROID {
                    Some(Position::new(x, y))
                } else {
                    None
                }
            })
        })
        .collect();

    part1(&asteroids);
    part2(&asteroids);

    Ok(())
}

/// Find the best location for a new monitoring station. How many other
/// asteroids can be detected from that location?
fn part1(asteroids: &[Position]) {
    let part1 = asteroids
        .iter()
        .map(|&station| {
            asteroids
                .iter()
                .filter_map(|&asteroid| station.reduced_direction(asteroid))
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap_or(0);

    println!("Part 1: {}", part1);
}

/// The Elves are placing bets on which will be the 200th asteroid to be
/// vaporized. Win the bet by determining which asteroid that will be; what do
/// you get if you multiply its X coordinate by 100 and then add its Y
/// coordinate? (For example, 8,2 becomes 802.)
fn part2(asteroids: &[Position]) {
    let best = *asteroids
        .iter()
        .max_by_key(|&&station| {
            asteroids
                .iter()
                .filter_map(|&asteroid| station.reduced_direction(asteroid))
                .collect::<HashSet<_>>()
                .len()
        })
        .unwrap();

    // Not my best work. Bin the directions from the best location so we can
    // "sort" them into layers. Then, it's just a matter of ensuring we run
    // through the first layer before going on to the second one, etc.
    let part2 = best
        + asteroids
            .iter()
            .filter(|&&position| position != best)
            .fold(
                BTreeMap::<Direction, BTreeSet<Direction>>::new(),
                |mut acc, &asteroid| {
                    let d = best.direction(asteroid).unwrap();
                    acc.entry(d.reduce()).or_default().insert(d);
                    acc
                },
            )
            .into_iter()
            .enumerate()
            .flat_map(|(fire_order, (_, sats))| {
                sats.into_iter()
                    .enumerate()
                    .map(move |(layer, sat)| (layer, fire_order, sat))
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .nth(199)
            .unwrap()
            .2;

    println!("Part 2: {}", part2.x * 100 + part2.y);
}
