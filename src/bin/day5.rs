use advent_of_code_2019::intcode::{read_program, Computer};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT_PATH: &str = "inputs/day5.txt";

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;
    Computer::new().load_program(program).execute()?;
    Ok(())
}
