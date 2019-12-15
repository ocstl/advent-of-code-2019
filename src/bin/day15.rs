use advent_of_code_2019::intcode::{read_program, Computer, IntCodeError, Program, Value};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

const INPUT_PATH: &str = "inputs/day15.txt";
const MOVEMENTS: [Movement; 4] = [
    Movement::North,
    Movement::South,
    Movement::West,
    Movement::East,
];

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type SystemMap = HashMap<Position, Status>;

#[derive(Debug, Clone, Copy)]
#[repr(isize)]
enum Movement {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl std::ops::Neg for Movement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Movement::North => Movement::South,
            Movement::South => Movement::North,
            Movement::West => Movement::East,
            Movement::East => Movement::West,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(isize, isize);

impl std::ops::Add<Movement> for Position {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, movement: Movement) -> Self::Output {
        match movement {
            Movement::North => Position(self.0, self.1 - 1),
            Movement::South => Position(self.0, self.1 + 1),
            Movement::West => Position(self.0 - 1, self.1),
            Movement::East => Position(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Wall,
    Open,
    OxygenSystem,
}

impl From<Value> for Status {
    fn from(value: Value) -> Self {
        match value {
            0 => Status::Wall,
            1 => Status::Open,
            2 => Status::OxygenSystem,
            _ => unimplemented!(),
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;
    let map = map(program)?;
    part1(&map);
    part2(&map);
    Ok(())
}

fn map(program: Program) -> Result<SystemMap> {
    let (computer, tx, rx) = Computer::new();

    std::thread::spawn(move || -> std::result::Result<(), IntCodeError> {
        let mut computer = computer;
        computer.load_program(program).execute()?;
        Ok(())
    });

    let mut map = SystemMap::new();
    let mut position = Position::default();
    map.insert(position, Status::Open);
    let mut movements = Vec::new();

    loop {
        // Find an open direction. If none (`None`) can be found, backtrack.
        if let Some(&d) = MOVEMENTS
            .iter()
            .find(|&&d| map.get(&(position + d)).is_none())
        {
            tx.send(d as Value)?;
            match Status::from(rx.recv()?) {
                Status::Wall => {
                    map.insert(position + d, Status::Wall);
                }
                x => {
                    position = position + d;
                    movements.push(d);
                    map.insert(position, x);
                }
            }
        } else {
            if let Some(d) = movements.pop() {
                tx.send(-d as Value)?;
                position = position + (-d);
                // Remember to eat the output (which is `Status::Open`).
                rx.recv()?;
            } else {
                break;
            }
        }
    }

    tx.send(0)?;
    Ok(map)
}

/// What is the fewest number of movement commands required to move the repair
/// droid from its starting position to the location of the oxygen system?
fn part1(map: &SystemMap) {
    // BFS.
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0_u32), Position::default()));

    let mut visited = HashSet::new();

    while let Some((Reverse(steps), position)) = queue.pop() {
        visited.insert(position);
        let new_positions = MOVEMENTS
            .iter()
            .filter_map(|&d| Some(position + d).filter(|p| !visited.contains(p)));

        for new_pos in new_positions {
            match map.get(&new_pos) {
                Some(&Status::Open) => queue.push((Reverse(steps + 1), new_pos)),
                Some(&Status::OxygenSystem) => {
                    println!("Part 1: {}", steps + 1);
                    break;
                }
                _ => (),
            }
        }
    }
}

/// Use the repair droid to get a complete map of the area. How many minutes
/// will it take to fill with oxygen?
fn part2(map: &SystemMap) {
    // BFS again.
    let mut queue = BinaryHeap::new();
    let oxygen_position = map
        .iter()
        .find_map(|(&pos, &status)| {
            if status == Status::OxygenSystem {
                Some(pos)
            } else {
                None
            }
        })
        .unwrap();
    queue.push((Reverse(0_u32), oxygen_position));

    let mut visited = HashMap::new();

    while let Some((Reverse(steps), position)) = queue.pop() {
        visited.insert(position, steps);
        let new_positions = MOVEMENTS
            .iter()
            .filter_map(|&d| Some(position + d).filter(|p| !visited.contains_key(p)));

        for new_pos in new_positions {
            if let Some(&Status::Open) = map.get(&new_pos) {
                queue.push((Reverse(steps + 1), new_pos))
            }
        }
    }

    let part2 = *visited.values().max().unwrap_or(&0);
    println!("Part 2: {}", part2);
}
