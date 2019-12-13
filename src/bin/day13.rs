use advent_of_code_2019::intcode::{read_program, Computer, IntCodeError, Program};
use std::collections::HashMap;
use std::sync::mpsc::{self, TryRecvError};

const INPUT_PATH: &str = "inputs/day13.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Score(isize),
}

impl From<isize> for Tile {
    fn from(value: isize) -> Self {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => Tile::Score(value),
        }
    }
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => '*',
            Tile::Paddle => '=',
            Tile::Ball => '.',
            Tile::Score(_) => ' ',
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

/// Start the game. How many block tiles are on the screen when the game exits?
fn part1(program: Program) -> Result<()> {
    let (_, r1) = mpsc::channel();
    let (t2, r2) = mpsc::channel();

    std::thread::spawn(move || -> std::result::Result<(), IntCodeError> {
        Computer::with_mpsc(r1, t2)
            .load_program(program)
            .execute()?;
        Ok(())
    });
    let mut screen = HashMap::new();

    while let Ok(x) = r2.recv() {
        let y = r2.recv()?;
        let tile = Tile::from(r2.recv()?);
        screen.insert((x, y), tile);
    }

    let part1 = screen.values().filter(|&&tile| tile == Tile::Block).count();
    dbg!(screen.get(&(-1, 0)));

    println!("Part 1: {}", part1);
    Ok(())
}

/// Beat the game by breaking all the blocks. What is your score after the last
/// block is broken?
fn part2(program: Program) -> Result<()> {
    let (t1, r1) = mpsc::channel();
    let (t2, r2) = mpsc::channel();

    std::thread::spawn(move || -> std::result::Result<(), IntCodeError> {
        let mut program = program;
        program[0] = 2;
        Computer::with_mpsc(r1, t2)
            .load_program(program)
            .execute()?;
        Ok(())
    });

    // To make sure the program is ready for input.
    std::thread::sleep(std::time::Duration::from_millis(10));

    let mut screen = HashMap::new();
    loop {
        match r2.try_recv() {
            Ok(x) => {
                let y = r2.recv()?;
                let tile = Tile::from(r2.recv()?);
                screen.insert((x, y), tile);
            }
            Err(TryRecvError::Empty) => {
                let ball_x = screen
                    .iter()
                    .find_map(|((x, _), &tile)| if tile == Tile::Ball { Some(*x) } else { None })
                    .unwrap();

                let paddle_x = screen
                    .iter()
                    .find_map(|((x, _), &tile)| if tile == Tile::Paddle { Some(*x) } else { None })
                    .unwrap();

                t1.send((ball_x - paddle_x).signum())?;

                // Print out the screen. Alternatively, we would need to wait a
                // bit for the arcade to start sending new output.
                let (max_x, max_y) = screen
                    .keys()
                    .fold((0, 0), |acc, key| (acc.0.max(key.0), acc.1.max(key.1)));

                for idy in 0..=max_y {
                    let line = (0..=max_x)
                        .map(|idx| char::from(*screen.get(&(idx, idy)).unwrap_or(&Tile::Empty)))
                        .collect::<String>();
                    println!("{}", line);
                }
            }
            Err(TryRecvError::Disconnected) => break,
        }
    }

    if let Some(Tile::Score(part2)) = screen.get(&(-1, 0)) {
        println!("Part 2: {}", part2);
    };

    Ok(())
}
