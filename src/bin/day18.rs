use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
const INPUT_PATH: &str = "inputs/day18.txt";

type Keys = BTreeSet<char>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type State = (Reverse<u32>, Position, Keys);
type Vault = HashMap<Position, Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    Entrance,
    Door(char),
    Key(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            '@' => Tile::Entrance,
            _ if c.is_ascii_uppercase() => Tile::Door(c.to_ascii_lowercase()),
            _ if c.is_ascii_lowercase() => Tile::Key(c),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(usize, usize);

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

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let vault = to_vault(&input);
    part1(&vault);
    part2(&vault);
    Ok(())
}

/// How many steps is the shortest path that collects all of the keys?
fn part1(vault: &Vault) {
    let entrance = *vault
        .iter()
        .find_map(|(k, v)| if v == &Tile::Entrance { Some(k) } else { None })
        .unwrap();

    let nbr_keys = vault
        .values()
        .filter_map(|tile| {
            if let Tile::Key(x) = tile {
                Some(x)
            } else {
                None
            }
        })
        .count();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), entrance, Keys::new()));

    let mut visited = HashSet::new();

    while let Some(state) = queue.pop() {
        if state.2.len() == nbr_keys {
            let Reverse(steps) = state.0;
            println!("Part 1: {}", steps);
            break;
        }

        if !visited.insert((state.1, state.2.clone())) {
            continue;
        }

        queue.extend(explore_vault(vault, state));
    }
}

/// Update your map to instead use the correct data:
///
/// ...     @#@
/// .@. =>  ###
/// ...     @#@
///
/// After updating your map and using the remote-controlled robots, what is the
/// fewest steps necessary to collect all of the keys?
fn part2(vault: &Vault) {
    let mut vault = vault.clone();

    // Update the map.
    let entrance = *vault
        .iter()
        .find_map(|(k, v)| if v == &Tile::Entrance { Some(k) } else { None })
        .unwrap();

    let entrances = [
        Position(entrance.0 - 1, entrance.1 - 1),
        Position(entrance.0 + 1, entrance.1 - 1),
        Position(entrance.0 - 1, entrance.1 + 1),
        Position(entrance.0 + 1, entrance.1 + 1),
    ];

    vault.insert(entrances[0], Tile::Entrance);
    vault.insert(entrances[1], Tile::Entrance);
    vault.insert(entrances[2], Tile::Entrance);
    vault.insert(entrances[3], Tile::Entrance);
    vault.insert(entrance, Tile::Wall);
    for &direction in DIRECTIONS.iter() {
        vault.insert(entrance + direction, Tile::Wall);
    }

    let nbr_keys = vault
        .values()
        .filter_map(|tile| {
            if let Tile::Key(x) = tile {
                Some(x)
            } else {
                None
            }
        })
        .count();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), entrances, Keys::new()));

    let mut visited = HashSet::new();

    while let Some((Reverse(steps), positions, keys)) = queue.pop() {
        if keys.len() == nbr_keys {
            println!("Part 2: {}", steps);
            break;
        }

        if !visited.insert((positions, keys.clone())) {
            continue;
        }

        for idx in 0..4 {
            let state = (Reverse(steps), positions[idx], keys.clone());
            for (steps, new_position, keys) in explore_vault(&vault, state) {
                let mut new_positions = positions;
                new_positions[idx] = new_position;
                queue.push((steps, new_positions, keys));
            }
        }
    }
}

fn to_vault(input: &str) -> Vault {
    input
        .lines()
        .enumerate()
        .flat_map(|(idy, line)| {
            line.char_indices()
                .map(move |(idx, tile)| (Position(idx, idy), Tile::from(tile)))
        })
        .collect()
}

/// We're only interested in state changes with new keys.
fn explore_vault(vault: &Vault, state: State) -> Vec<State> {
    let (Reverse(steps), position, keys) = state;

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(steps), position, keys));

    let mut new_states = Vec::new();
    let mut visited = HashSet::new();

    while let Some((Reverse(steps), position, keys)) = queue.pop() {
        if !visited.insert(position) {
            continue;
        }

        for new_position in DIRECTIONS.iter().map(|&d| position + d) {
            match vault.get(&new_position).unwrap() {
                Tile::Wall => (),
                Tile::Open => queue.push((Reverse(steps + 1), new_position, keys.clone())),
                Tile::Entrance => queue.push((Reverse(steps + 1), new_position, keys.clone())),
                Tile::Door(x) if keys.contains(x) => {
                    queue.push((Reverse(steps + 1), new_position, keys.clone()))
                }
                Tile::Door(_) => (),
                Tile::Key(x) => {
                    let mut new_keys = keys.clone();
                    new_keys.insert(*x);

                    if !keys.contains(x) {
                        new_states.push((Reverse(steps + 1), new_position, new_keys.clone()));
                    }

                    queue.push((Reverse(steps + 1), new_position, new_keys));
                }
            }
        }
    }

    new_states
}
