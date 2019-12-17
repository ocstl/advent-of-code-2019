use advent_of_code_2019::intcode::{read_program, Computer, Program, Value};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

const INPUT_PATH: &str = "inputs/day17.txt";
const SCAFFOLD: isize = '#' as isize;
const OPEN: isize = '.' as isize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl Position {
    fn alignment_parameter(self) -> isize {
        self.0 * self.1
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Up => Position(self.0, self.1 - 1),
            Direction::Down => Position(self.0, self.1 + 1),
            Direction::Left => Position(self.0 - 1, self.1),
            Direction::Right => Position(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone)]
struct Map(HashMap<Position, Value>);

impl Map {
    fn get(&self, position: Position) -> Option<Value> {
        self.0.get(&position).copied()
    }

    fn iter(&self) -> std::collections::hash_map::Iter<Position, Value> {
        self.0.iter()
    }
}

impl From<&Receiver<Value>> for Map {
    fn from(rx: &Receiver<Value>) -> Self {
        let mut position = Position::default();
        let mut map = HashMap::new();

        for value in rx.try_iter() {
            // New line.
            if value == 10 {
                position = Position(0, position.1 + 1);
            } else {
                map.insert(position, value);
                position = position + Direction::Right;
            }
        }

        Map(map)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(input.as_str())?;

    part1(program.clone())?;
    part2(program)?;
    Ok(())
}

/// Run your ASCII program. What is the sum of the alignment parameters for the
/// scaffold intersections?
fn part1(program: Program) -> Result<()> {
    let (mut computer, _, rx) = Computer::new();
    computer.load_program(program).execute()?;

    let map = Map::from(&rx);
    let part1: isize = map.iter()
        .filter_map(|(&pos, &value)|
            // Use `!= OPEN`, since the bot is on a scaffold.
            if value != OPEN
                    && map.get(pos + Direction::Up) != Some(OPEN)
                    && map.get(pos + Direction::Down) != Some(OPEN)
                    && map.get(pos + Direction::Left) != Some(OPEN)
                    && map.get(pos + Direction::Right) != Some(OPEN) {
                Some(pos.alignment_parameter())
            } else {
                None
            }
        ).sum();

    println!("Part 1: {}", part1);
    Ok(())
}

/// After visiting every part of the scaffold at least once, how much dust does
/// the vacuum robot report it has collected?
fn part2(mut program: Program) -> Result<()> {
    // Force the vacuum robot to wake up by changing the value in your ASCII
    // program at address 0 from 1 to 2.
    program[0] = 2;

    let (mut computer, tx, rx) = Computer::new();

    // Working it out by hand, we get:
    let main = "A,C,A,C,B,C,B,A,C,B";
    let fn_a = "R,4,R,10,R,8,R,4";
    let fn_b = "R,4,L,12,R,6,L,12";
    let fn_c = "R,10,R,6,R,4";

    for c in format!("{}\n{}\n{}\n{}\n", main, fn_a, fn_b, fn_c).chars() {
        tx.send(c as isize)?;
    }

    // We don't want the continuous video feed.
    tx.send('n' as isize)?;
    tx.send('\n' as isize)?;

    computer.load_program(program).execute()?;

    let part2 = rx.try_iter().last().unwrap();
    println!("Part 2: {}", part2);
    Ok(())
}
