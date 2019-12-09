use advent_of_code_2019::intcode::{read_program, Computer, Program};

const INPUT_PATH: &str = "inputs/day9.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;
    part1(program.clone())?;
    part2(program)
}

/// Once your Intcode computer is fully functional, the BOOST program should
/// report no malfunctioning opcodes when run in test mode; it should only
/// output a single value, the BOOST keycode. What BOOST keycode does it
/// produce?
fn part1(program: Program) -> Result<()> {
    let (mut computer, tx, rx) = Computer::new();
    tx.send(1)?;
    computer.load_program(program).execute()?;

    let part1 = rx.recv()?;
    println!("Part 1: {}", part1);
    Ok(())
}

/// The program runs in sensor boost mode by providing the input instruction the
/// value 2. Once run, it will boost the sensors automatically, but it might
/// take a few seconds to complete the operation on slower hardware. In sensor
/// boost mode, the program will output a single value: the coordinates of the
/// distress signal.
///
/// Run the BOOST program in sensor boost mode. What are the coordinates of the
/// distress signal?
fn part2(program: Program) -> Result<()> {
    let (mut computer, tx, rx) = Computer::new();
    tx.send(2)?;
    computer.load_program(program).execute()?;

    let part2 = rx.recv()?;
    println!("Part 2: {}", part2);
    Ok(())
}
