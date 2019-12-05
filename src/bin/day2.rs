use advent_of_code_2019::intcode::{read_program, Computer, Program};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT_PATH: &str = "inputs/day2.txt";

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;
    part1(program.clone())?;
    part2(program)?;
    Ok(())
}

/// Once you have a working computer, the first step is to restore the gravity
/// assist program (your puzzle input) to the "1202 program alarm" state it had
/// just before the last computer caught fire. To do this, before running the
/// program, replace position 1 with the value 12 and replace position 2 with
/// the value 2. What value is left at position 0 after the program halts?
fn part1(mut program: Program) -> Result<()> {
    program[1] = 12;
    program[2] = 2;

    let part1 = *Computer::new()
        .load_program(program)
        .execute()?
        .memory()
        .first()
        .unwrap();

    println!("Part 1: {}", part1);
    Ok(())
}

/// Find the input noun and verb that cause the program to produce the output
/// 19690720. What is 100 * noun + verb? (For example, if noun = 12 and
/// verb = 2, the answer would be 1202.)
fn part2(program: Program) -> Result<()> {
    let mut computer = Computer::new();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut new_program = program.to_owned();
            new_program[1] = noun;
            new_program[2] = verb;
            computer.load_program(new_program).execute()?;

            if computer.memory().first() == Some(&19_690_720) {
                println!("Part 2: {}", noun * 100 + verb);
                break;
            }
        }
    }

    Ok(())
}
