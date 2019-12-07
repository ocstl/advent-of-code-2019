use advent_of_code_2019::intcode::{read_program, Computer, Program};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT_PATH: &str = "inputs/day5.txt";

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;
    part1(program.clone())?;
    part2(program)?;
    Ok(())
}

/// After providing 1 to the only input instruction and passing all the tests,
/// what diagnostic code does the program produce?
fn part1(program: Program) -> Result<()> {
    let (mut computer, tx, rx) = Computer::new();
    tx.send(1)?;
    computer.load_program(program).execute()?;

    let part1 = rx.iter().filter(|&v| v != 0).next().unwrap_or(0);
    println!("Part 1: {}", part1);
    Ok(())
}

// What is the diagnostic code for system ID 5?
fn part2(program: Program) -> Result<()> {
    let (mut computer, tx, rx) = Computer::new();
    tx.send(5)?;
    computer.load_program(program).execute()?;

    let part2 = rx.iter().filter(|&v| v != 0).next().unwrap_or(0);
    println!("Part 2: {}", part2);
    Ok(())
}
