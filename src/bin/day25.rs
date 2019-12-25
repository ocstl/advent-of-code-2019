use advent_of_code_2019::intcode::{read_program, Computer, IntCodeError, Program};
use std::io::{self, Write};

const INPUT_PATH: &str = "inputs/day25.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;
    part1(program)?;

    Ok(())
}

fn part1(program: Program) -> Result<()> {
    let (mut computer, tx, rx) = Computer::new();
    computer.load_program(program);

    std::thread::spawn(move || -> std::result::Result<(), IntCodeError> {
        let mut computer = computer;
        computer.execute()?;
        Ok(())
    });

    loop {
        let output: Vec<u8> = rx.try_iter().map(|value| value as u8).collect();
        if !output.is_empty() {
            io::stdout().write_all(&output)?;
            continue;
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        for byte in input.bytes() {
            tx.send(byte as isize)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
