use std::convert::TryFrom;
use std::sync::mpsc;

pub type Address = isize;
pub type Memory = Vec<isize>;
type Parameters = (ParameterMode, ParameterMode, ParameterMode);
pub type Program = Vec<isize>;
pub type Value = isize;

pub fn read_program(text: &str) -> Result<Program, std::num::ParseIntError> {
    let mut program = Vec::new();
    for value in text.trim().split(',') {
        program.push(value.parse::<isize>()?);
    }

    Ok(program)
}

#[derive(Debug, Clone, Copy)]
pub enum IntCodeError {
    Halt,
    InvalidAddress(Address),
    InvalidOpCode(Value),
    InvalidParameterMode(Value),
    ReadError,
    WriteError,
    WriteImmediateMode,
}

impl std::fmt::Display for IntCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for IntCodeError {}

type IntCodeResult<T> = std::result::Result<T, IntCodeError>;

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl TryFrom<Value> for Opcode {
    type Error = IntCodeError;

    fn try_from(value: Value) -> IntCodeResult<Opcode> {
        Ok(match value {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            _ => return Err(IntCodeError::InvalidOpCode(value)),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl TryFrom<Value> for ParameterMode {
    type Error = IntCodeError;

    fn try_from(value: Value) -> IntCodeResult<Self> {
        Ok(match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => return Err(IntCodeError::InvalidParameterMode(value)),
        })
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: Opcode,
    parameters: (ParameterMode, ParameterMode, ParameterMode),
}

impl TryFrom<Value> for Instruction {
    type Error = IntCodeError;

    fn try_from(value: Value) -> IntCodeResult<Self> {
        Ok(Instruction {
            opcode: Opcode::try_from(value % 100)?,
            parameters: (
                ParameterMode::try_from((value / 100) % 10)?,
                ParameterMode::try_from((value / 1000) % 10)?,
                ParameterMode::try_from(value / 10_000)?,
            ),
        })
    }
}

#[derive(Debug)]
pub struct Computer {
    memory: Memory,
    instruction_pointer: Address,
    receiver: mpsc::Receiver<Value>,
    sender: mpsc::Sender<Value>,
}

impl Computer {
    pub fn new() -> (Self, mpsc::Sender<Value>, mpsc::Receiver<Value>) {
        let (tx, receiver) = mpsc::channel();
        let (sender, rx) = mpsc::channel();
        let computer = Computer {
            memory: Memory::new(),
            instruction_pointer: 0,
            receiver,
            sender,
        };
        (computer, tx, rx)
    }

    pub fn with_mpsc(receiver: mpsc::Receiver<Value>, sender: mpsc::Sender<Value>) -> Self {
        Computer {
            memory: Memory::new(),
            instruction_pointer: 0,
            receiver,
            sender,
        }
    }

    pub fn load_program(&mut self, program: Program) -> &mut Self {
        self.memory = program;
        self.reset();
        self
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn execute(&mut self) -> IntCodeResult<&mut Self> {
        let mut result = Ok(());
        while let Ok(_) = result {
            let Instruction { parameters, opcode } = self.read_instruction()?;
            result = match opcode {
                Opcode::Add => self.add(parameters),
                Opcode::Multiply => self.multiply(parameters),
                Opcode::Input => self.input(parameters),
                Opcode::Output => self.output(parameters),
                Opcode::JumpIfTrue => self.jump_if_true(parameters),
                Opcode::JumpIfFalse => self.jump_if_false(parameters),
                Opcode::LessThan => self.less_than(parameters),
                Opcode::Equals => self.equals(parameters),
                Opcode::Halt => Err(IntCodeError::Halt),
            };
        }

        Ok(self)
    }

    pub fn reset(&mut self) -> &mut Self {
        self.instruction_pointer = 0;
        self
    }

    fn read_address(&self, address: Address) -> IntCodeResult<Value> {
        if !address.is_negative() && address < self.memory.len() as isize {
            Ok(self.memory[address as usize])
        } else {
            Err(IntCodeError::InvalidAddress(address))
        }
    }

    fn write_address(&mut self, address: Address, value: Value) -> IntCodeResult<()> {
        if !address.is_negative() && address < self.memory.len() as isize {
            self.memory[address as usize] = value;
            Ok(())
        } else {
            Err(IntCodeError::InvalidAddress(address))
        }
    }

    fn read_next(&mut self, mode: ParameterMode) -> IntCodeResult<Value> {
        let mut result = self.read_address(self.instruction_pointer)?;
        self.instruction_pointer += 1;

        if mode == ParameterMode::Position {
            result = self.read_address(result)?;
        }

        Ok(result)
    }

    fn write_next(&mut self, value: Value, mode: ParameterMode) -> IntCodeResult<()> {
        if mode == ParameterMode::Immediate {
            Err(IntCodeError::WriteImmediateMode)
        } else {
            self.read_next(ParameterMode::Immediate)
                .and_then(|address| self.write_address(address, value))
        }
    }

    fn read_instruction(&mut self) -> IntCodeResult<Instruction> {
        self.read_next(ParameterMode::Immediate)
            .and_then(Instruction::try_from)
    }

    fn add(&mut self, parameters: Parameters) -> IntCodeResult<()> {
        let a = self.read_next(parameters.0)?;
        let b = self.read_next(parameters.1)?;
        self.write_next(a + b, parameters.2)
    }

    fn multiply(&mut self, parameters: Parameters) -> IntCodeResult<()> {
        let a = self.read_next(parameters.0)?;
        let b = self.read_next(parameters.1)?;
        self.write_next(a * b, parameters.2)
    }

    fn input(&mut self, parameters: Parameters) -> IntCodeResult<()> {
        let value = self.receiver.recv().or(Err(IntCodeError::ReadError))?;
        self.write_next(value, parameters.0)
    }

    fn output(&mut self, parameters: Parameters) -> IntCodeResult<()> {
        let value = self.read_next(parameters.0)?;
        self.sender.send(value).or(Err(IntCodeError::WriteError))
    }

    fn jump_if_true(&mut self, parameters: Parameters) -> IntCodeResult<()> {
        let a = self.read_next(parameters.0)?;
        let b = self.read_next(parameters.1)?;
        if a != 0 {
            self.instruction_pointer = b;
        }

        Ok(())
    }

    fn jump_if_false(&mut self, parameters: Parameters) -> IntCodeResult<()> {
        let a = self.read_next(parameters.0)?;
        let b = self.read_next(parameters.1)?;
        if a == 0 {
            self.instruction_pointer = b;
        }

        Ok(())
    }

    fn less_than(&mut self, parameters: Parameters) -> IntCodeResult<()> {
        let a = self.read_next(parameters.0)?;
        let b = self.read_next(parameters.1)?;

        self.write_next(if a < b { 1 } else { 0 }, parameters.2)
    }

    fn equals(&mut self, parameters: Parameters) -> IntCodeResult<()> {
        let a = self.read_next(parameters.0)?;
        let b = self.read_next(parameters.1)?;

        self.write_next(if a == b { 1 } else { 0 }, parameters.2)
    }
}
