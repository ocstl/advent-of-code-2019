use advent_of_code_2019::intcode::{read_program, Computer, IntCodeError, Program, Value};

const INPUT_PATH: &str = "inputs/day21.txt";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;
    part1(program.clone())?;
    part2(program)?;

    Ok(())
}

/// Program the springdroid with logic that allows it to survey the hull without
/// falling into space. What amount of hull damage does it report?
fn part1(program: Program) -> Result<()> {
    let (computer, tx, rx) = Computer::new();
    let send = |instruction: &str| send_instruction(instruction, tx.clone());

    std::thread::spawn(move || -> std::result::Result<(), IntCodeError> {
        let mut computer = computer;
        computer.load_program(program).execute()?;
        Ok(())
    });

    // Jump is there is no ground in front.
    send("NOT A J")?;

    // Or if there is a hole coming up.
    send("NOT C T")?;
    send("OR T J")?;

    // As long as there is ground to land on.
    send("AND D J")?;

    // Start walking.
    send("WALK")?;

    let result: Vec<Value> = rx.iter().collect();
    let part1 = *result.last().unwrap();
    if part1 > std::u8::MAX as isize {
        println!("Part 1: {}", part1);
    } else {
        let s: String = result
            .into_iter()
            .map(|value| (value as u8) as char)
            .collect();
        println!("{}", s);
    }

    Ok(())
}

fn part2(program: Program) -> Result<()> {
    let (computer, tx, rx) = Computer::new();
    let send = |instruction: &str| send_instruction(instruction, tx.clone());

    std::thread::spawn(move || -> std::result::Result<(), IntCodeError> {
        let mut computer = computer;
        computer.load_program(program).execute()?;
        Ok(())
    });

    // Jump is there is a hole coming soon.
    send("OR A J")?;
    send("AND B J")?;
    send("AND C J")?;
    send("NOT J J")?;

    // As long as there is ground to land on.
    send("AND D J")?;

    // As long as we can step forward or jump afterwards.
    send("OR E T")?;
    send("OR H T")?;
    send("AND T J")?;

    // Start running.
    send("RUN")?;

    let result: Vec<Value> = rx.iter().collect();
    let part2 = *result.last().unwrap();
    if part2 > std::u8::MAX as isize {
        println!("Part 2: {}", part2);
    } else {
        let s: String = result
            .into_iter()
            .map(|value| (value as u8) as char)
            .collect();
        println!("{}", s);
    }

    Ok(())
}

fn send_instruction(instruction: &str, tx: std::sync::mpsc::Sender<Value>) -> Result<()> {
    for c in instruction.chars() {
        tx.send(c as Value)?;
    }

    tx.send('\n' as Value)?;
    Ok(())
}
