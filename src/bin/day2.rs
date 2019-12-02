use std::convert::TryFrom;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Instruction = (Opcode, usize, usize, usize);

const INPUT_PATH: &str = "inputs/day2.txt";

enum Opcode {
    Add,
    Multiply,
    Halt,
}

impl From<usize> for Opcode {
    fn from(input: usize) -> Self {
        match input {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::Halt,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    data: Vec<usize>,
}

impl Program {
    fn get(&self, idx: usize) -> Option<usize> {
        self.data.get(idx).copied()
    }

    fn set(&mut self, idx: usize, value: usize) {
        self.data[idx] = value;
    }

    fn instruction(&self, idx: usize) -> Option<Instruction> {
        Some((
            Opcode::from(self.get(idx)?),
            self.get(idx + 1)?,
            self.get(idx + 2)?,
            self.get(idx + 3)?,
        ))
    }

    fn run_program(&mut self) {
        for ptr in (0..).step_by(4) {
            match self.instruction(ptr) {
                Some((Opcode::Add, a, b, c)) => {
                    self.set(c, self.get(a).unwrap() + self.get(b).unwrap())
                }
                Some((Opcode::Multiply, a, b, c)) => {
                    self.set(c, self.get(a).unwrap() * self.get(b).unwrap())
                }
                Some((Opcode::Halt, ..)) => break,
                None => break,
            }
        }
    }
}

impl TryFrom<&str> for Program {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: &str) -> Result<Self> {
        let mut data = Vec::new();
        for value in input.trim().split(',') {
            data.push(value.parse::<usize>()?);
        }

        Ok(Program { data })
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let program = Program::try_from(input.as_str())?;
    part1(program.clone());
    part2(program);
    Ok(())
}

/// Once you have a working computer, the first step is to restore the gravity
/// assist program (your puzzle input) to the "1202 program alarm" state it had
/// just before the last computer caught fire. To do this, before running the
/// program, replace position 1 with the value 12 and replace position 2 with
/// the value 2. What value is left at position 0 after the program halts?
fn part1(mut program: Program) {
    program.set(1, 12);
    program.set(2, 2);
    program.run_program();

    println!("Part 1: {}", program.get(0).unwrap());
}

/// Find the input noun and verb that cause the program to produce the output
/// 19690720. What is 100 * noun + verb? (For example, if noun = 12 and
/// verb = 2, the answer would be 1202.)
fn part2(program: Program) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut new_program = program.clone();
            new_program.set(1, noun);
            new_program.set(2, verb);
            new_program.run_program();
            if new_program.get(0) == Some(19_690_720) {
                println!("Part 2: {}", noun * 100 + verb);
                break;
            }
        }
    }
}
