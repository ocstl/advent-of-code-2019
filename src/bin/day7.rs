use advent_of_code_2019::intcode::{read_program, Computer, IntCodeError, Program};
use permutohedron::Heap;
use std::sync::mpsc;
use std::thread;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT_PATH: &str = "inputs/day7.txt";
const PHASE_SETTINGS: isize = 5;

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = read_program(&input)?;
    part1(program.clone())?;
    part2(program)?;
    Ok(())
}

/// Try every combination of phase settings on the amplifiers. What is the
/// highest signal that can be sent to the thrusters?
fn part1(program: Program) -> Result<()> {
    let mut settings: Vec<isize> = (0..PHASE_SETTINGS).collect();
    let heap = Heap::new(&mut settings);

    let part1 = heap
        .map(|s| -> Result<isize> {
            let mut signal = 0;
            for setting in s {
                let (mut computer, tx, rx) = Computer::new();
                tx.send(setting)?;
                tx.send(signal)?;
                computer.load_program(program.clone()).execute()?;
                signal = rx.recv().expect("No output.");
            }
            Ok(signal)
        })
        .collect::<Result<Vec<isize>>>()?
        .into_iter()
        .max()
        .unwrap_or(0);

    println!("Part 1: {}", part1);
    Ok(())
}

/// Try every combination of the new phase settings on the amplifier feedback
/// loop. What is the highest signal that can be sent to the thrusters?
fn part2(program: Program) -> Result<()> {
    let mut settings: Vec<isize> = (5..10).collect();
    let heap = Heap::new(&mut settings);

    let part2 = heap
        .map(|s| -> Result<isize> {
            let mut senders = Vec::new();
            let mut receivers = Vec::new();
            for &setting in &s {
                let (t1, r1) = mpsc::channel();
                let (t2, r2) = mpsc::channel();
                t1.send(setting);
                senders.push(t1);
                receivers.push(r2);

                let program = program.clone();
                thread::spawn(move || -> std::result::Result<(), IntCodeError> {
                    Computer::with_mpsc(r1, t2).load_program(program).execute()?;
                    Ok(())
                });
            }

            // Tried to mem::swap the receivers, but no dice. So, this feels
            // dumb, but at least it works.
            let mut value = 0;
            let mut idx = 0;
            while senders[idx].send(value).is_ok() {
                value = receivers[idx].recv()?;
                idx = (idx + 1) % s.len();
            }

            Ok(value)
        })
        .collect::<Result<Vec<isize>>>()?
        .into_iter()
        .max()
        .unwrap_or(0);

    println!("Part 2: {}", part2);
    Ok(())
}
