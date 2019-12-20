use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
const ENTRANCE: (char, char) = ('A', 'A');
const EXIT: (char, char) = ('Z', 'Z');
const INPUT_PATH: &str = "inputs/day20.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    Portal(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' | ' ' => Tile::Wall,
            '.' => Tile::Open,
            _ if c.is_ascii_uppercase() => Tile::Portal(c),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
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

#[derive(Debug, Clone)]
struct Maze {
    map: HashMap<Position, Tile>,
    portals: HashMap<(char, char), Vec<Position>>,
    entrance: Position,
    exit: Position,
    min_open_tile_coordinates: (usize, usize),
    max_open_tile_coordinates: (usize, usize),
}

impl Maze {
    fn reachable(&self, position: Position) -> impl Iterator<Item = (Position, isize)> + '_ {
        let level_change = if position.0 == self.min_open_tile_coordinates.0
            || position.0 == self.max_open_tile_coordinates.0
            || position.1 == self.min_open_tile_coordinates.1
            || position.1 == self.max_open_tile_coordinates.1
        {
            -1
        } else {
            1
        };

        DIRECTIONS
            .iter()
            .filter_map(move |&d| {
                let new_pos = position + d;
                match self.map.get(&new_pos).unwrap() {
                    Tile::Wall => None,
                    Tile::Open => Some((new_pos, 0)),
                    Tile::Portal(a) => {
                        if let Some(Tile::Portal(b)) = self.map.get(&(new_pos + d)) {
                            match d {
                                Direction::Up | Direction::Left => self
                                    .portals
                                    .get(&(*b, *a))
                                    .unwrap()
                                    .iter()
                                    .find_map(|&pos| {
                                        if pos != position {
                                            Some((pos, level_change))
                                        } else {
                                            None
                                        }
                                    }),
                                Direction::Down | Direction::Right => self
                                    .portals
                                    .get(&(*a, *b))
                                    .unwrap()
                                    .iter()
                                    .find_map(|&pos| {
                                        if pos != position {
                                            Some((pos, level_change))
                                        } else {
                                            None
                                        }
                                    }),
                            }
                        } else {
                            None
                        }
                    }
                }
            })
            .filter(move |&(pos, change)| change == 0 || pos != self.entrance || pos != self.exit)
    }

    fn entrance(&self) -> Position {
        self.entrance
    }

    fn exit(&self) -> Position {
        self.exit
    }
}

impl From<&str> for Maze {
    fn from(input: &str) -> Self {
        let map: HashMap<Position, Tile> = input
            .lines()
            .enumerate()
            .flat_map(|(idy, line)| {
                line.char_indices()
                    .map(move |(idx, tile)| (Position(idx, idy), Tile::from(tile)))
            })
            .collect();

        let portals = map
            .iter()
            .filter_map(|(&pos, &tile)| {
                // Ignore the upper and left borders (underflow issues), as well as
                // those tiles that are not labeled.
                if pos.0 == 0 || pos.1 == 0 {
                    return None;
                }

                if let Tile::Portal(a) = tile {
                    // If there is an open tile, the other label will be in the
                    // reverse direction. Just have to order them right.
                    if let Some(&d) = DIRECTIONS
                        .iter()
                        .find(|&&d| map.get(&(pos + d)) == Some(&Tile::Open))
                    {
                        if let Tile::Portal(b) = map.get(&(pos + (-d))).unwrap() {
                            match d {
                                Direction::Up | Direction::Left => Some(((a, *b), pos + d)),
                                Direction::Down | Direction::Right => Some(((*b, a), pos + d)),
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .fold(
                HashMap::<(char, char), Vec<Position>>::new(),
                |mut acc, portal| {
                    acc.entry(portal.0).or_default().push(portal.1);
                    acc
                },
            );

        let entrance = *portals.get(&ENTRANCE).unwrap().first().unwrap();
        let entrance = DIRECTIONS
            .iter()
            .find_map(|&d| {
                if map.get(&(entrance + d)) == Some(&Tile::Open) {
                    Some(entrance + d)
                } else {
                    None
                }
            })
            .unwrap();

        let exit = *portals.get(&EXIT).unwrap().first().unwrap();
        let exit = DIRECTIONS
            .iter()
            .find_map(|&d| {
                if map.get(&(exit + d)) == Some(&Tile::Open) {
                    Some(exit + d)
                } else {
                    None
                }
            })
            .unwrap();

        let open_tiles = map
            .iter()
            .filter_map(|(pos, &tile)| if tile == Tile::Open { Some(pos) } else { None });

        let min_x = open_tiles.clone().map(|pos| pos.0).min().unwrap();
        let min_y = open_tiles.clone().map(|pos| pos.1).min().unwrap();
        let max_x = open_tiles.clone().map(|pos| pos.0).max().unwrap();
        let max_y = open_tiles.clone().map(|pos| pos.1).max().unwrap();

        Maze {
            map,
            portals,
            entrance,
            exit,
            min_open_tile_coordinates: (min_x, min_y),
            max_open_tile_coordinates: (max_x, max_y),
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let maze = Maze::from(input.as_str());

    part1(&maze);
    part2(&maze);
    Ok(())
}

/// In your maze, how many steps does it take to get from the open tile marked
/// AA to the open tile marked ZZ?
fn part1(maze: &Maze) {
    let entrance = maze.entrance();
    let exit = maze.exit();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(2), entrance));

    let mut visited = HashSet::new();

    while let Some((Reverse(steps), position)) = queue.pop() {
        if position == exit {
            println!("Part 1: {}", steps);
            break;
        }

        if !visited.insert(position) {
            continue;
        }

        for (new_position, _) in maze.reachable(position) {
            queue.push((Reverse(steps + 1), new_position));
        }
    }
}

/// In your maze, when accounting for recursion, how many steps does it take to
/// get from the open tile marked AA to the open tile marked ZZ, both at the
/// outermost layer?
fn part2(maze: &Maze) {
    let entrance = maze.entrance();
    let exit = maze.exit();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(2), Reverse(0), entrance));

    let mut visited = HashSet::new();

    while let Some((Reverse(steps), Reverse(level), position)) = queue.pop() {
        if level == 0 && position == exit {
            println!("Part 2: {}", steps);
            break;
        }

        if !visited.insert((level, position)) {
            continue;
        }

        for (new_position, level_change) in maze.reachable(position) {
            if (level + level_change) >= 0 {
                queue.push((
                    Reverse(steps + 1),
                    Reverse(level + level_change),
                    new_position,
                ))
            }
        }
    }
}
