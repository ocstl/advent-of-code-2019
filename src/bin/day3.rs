use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

const INPUT_PATH: &str = "inputs/day3.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Wire = Vec<Point>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize);

impl Point {
    fn new() -> Self {
        Point(0, 0)
    }

    fn distance_to_origin(self) -> isize {
        self.0.abs() + self.1.abs()
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl From<char> for Direction {
    fn from(input: char) -> Self {
        match input {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = Self;

    fn add(self, d: Direction) -> Self {
        match d {
            Direction::Right => Point(self.0 + 1, self.1),
            Direction::Left => Point(self.0 - 1, self.1),
            Direction::Up => Point(self.0, self.1 + 1),
            Direction::Down => Point(self.0, self.1 - 1),
        }
    }
}

/// Opening the front panel reveals a jumble of wires. Specifically, two wires
/// are connected to a central port and extend outward on a grid. You trace the
/// path each wire takes as it leaves the central port, one wire per line of
/// text (your puzzle input).
fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let wires = into_wires(&input);
    part1(&wires)?;
    part2(&wires)?;
    Ok(())
}

fn into_wires(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split(',')
                .flat_map(instruction)
                .scan(Point::new(), |state, step| {
                    *state = *state + step;
                    Some(*state)
                })
                .collect()
        })
        .collect()
}

/// We're assuming a properly formatted input by calling `unwrap`, but error
/// handling in this case is gnarly.
fn instruction(input: &str) -> impl Iterator<Item = Direction> {
    let direction = Direction::from(input.chars().next().unwrap());
    let steps = input[1..].parse::<usize>().unwrap();
    std::iter::repeat(direction).take(steps)
}

/// What is the Manhattan distance from the central port to the closest
/// intersection?
fn part1(wires: &[Wire]) -> Result<()> {
    let wire1: HashSet<Point> =
        HashSet::from_iter(wires.first().ok_or("Missing first wire.")?.iter().copied());

    let closest_intersection = wires
        .get(1)
        .ok_or("Missing second wire.")?
        .iter()
        .filter_map(|p| Some(p.distance_to_origin()).filter(|_| wire1.contains(p)))
        .min()
        .ok_or("No intersection.")?;

    println!("Part 1: {}", closest_intersection);
    Ok(())
}

/// What is the fewest combined steps the wires must take to reach an
/// intersection?
fn part2(wires: &[Wire]) -> Result<()> {
    let wire1: HashMap<Point, usize> = wires
        .first()
        .ok_or("Missing first wire.")?
        .iter()
        .enumerate()
        // Since the values will be overwritten in case of collision, reverse
        // the order to retain only the first one.
        .rev()
        .map(|(idx, &p)| (p, idx))
        .collect();

    let fewest_steps = wires
        .get(1)
        .ok_or("Missing second wire.")?
        .iter()
        .enumerate()
        // Don't forget to adjust for 0-indexing (+1 for wire1, +1 for wire2).
        .filter_map(|(idx, p)| wire1.get(p).map(|&idy| idx + idy + 2))
        .min()
        .ok_or("No intersection.")?;

    println!("Part 2: {}", fewest_steps);
    Ok(())
}
