use advent_of_code_2019::intcode::{read_program, Computer, IntCodeError, Program};
use std::collections::HashMap;
use std::thread;

const INPUT_PATH: &str = "inputs/day11.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl From<isize> for Turn {
    fn from(value: isize) -> Self {
        match value {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, d: Direction) -> Self::Output {
        match d {
            Direction::Up => Position(self.0, self.1 - 1),
            Direction::Right => Position(self.0 + 1, self.1),
            Direction::Down => Position(self.0, self.1 + 1),
            Direction::Left => Position(self.0 - 1, self.1),
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;
    part1(program.clone())?;
    part2(program)?;

    Ok(())
}

/// Build a new emergency hull painting robot and run the Intcode program on it.
/// How many panels does it paint at least once?
fn part1(program: Program) -> Result<()> {
    let (computer, tx, rx) = Computer::new();

    thread::spawn(move || -> std::result::Result<(), IntCodeError> {
        let mut computer = computer;
        computer.load_program(program).execute()
    });

    let mut position = Position::default();
    let mut direction = Direction::Up;
    let mut map = HashMap::new();
    tx.send(0)?;
    while let Ok(color) = rx.recv() {
        map.insert(position, color);
        direction = match Turn::from(rx.recv()?) {
            Turn::Left => direction.turn_left(),
            Turn::Right => direction.turn_right(),
        };
        position = position + direction;
        tx.send(*map.get(&position).unwrap_or(&0))?;
    }

    let part1 = map.len();
    println!("Part 1: {}", part1);

    Ok(())
}

/// Based on the Space Law Space Brochure that the Space Police attached to one
/// of your windows, a valid registration identifier is always eight capital
/// letters. After starting the robot on a single white panel instead, what
/// registration identifier does it paint on your hull?
fn part2(program: Program) -> Result<()> {
    let (computer, tx, rx) = Computer::new();

    thread::spawn(move || -> std::result::Result<(), IntCodeError> {
        let mut computer = computer;
        computer.load_program(program).execute()
    });

    let mut position = Position::default();
    let mut direction = Direction::Up;
    let mut map = HashMap::new();
    tx.send(1)?;
    while let Ok(color) = rx.recv() {
        map.insert(position, color);
        direction = match Turn::from(rx.recv()?) {
            Turn::Left => direction.turn_left(),
            Turn::Right => direction.turn_right(),
        };
        position = position + direction;
        tx.send(*map.get(&position).unwrap_or(&0))?;
    }

    let (max_x, max_y) = map
        .keys()
        .fold((0, 0), |acc, key| (acc.0.max(key.0), acc.1.max(key.1)));

    println!("Part 2:");
    for idy in 0..=max_y {
        let line = (0..=max_x)
            .map(|idx| match map.get(&Position(idx, idy)).unwrap_or(&0) {
                0 => ' ',
                1 => '#',
                _ => unreachable!(),
            })
            .collect::<String>();
        println!("{}", line);
    }

    Ok(())
}
