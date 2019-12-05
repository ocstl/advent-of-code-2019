pub type Address = usize;
pub type Memory = Vec<usize>;
pub type Program = Vec<usize>;
pub type Value = usize;

pub fn read_program(text: &str) -> Result<Program, std::num::ParseIntError> {
    let mut program = Vec::new();
    for value in text.trim().split(',') {
        program.push(value.parse::<usize>()?);
    }

    Ok(program)
}

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

#[derive(Debug, Default, Clone)]
pub struct Computer {
    memory: Memory,
    instruction_pointer: usize,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            memory: Memory::new(),
            instruction_pointer: 0,
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

    pub fn execute(&mut self) -> &mut Self {
        while let Some(_) = {
            match self.read_opcode() {
                Some(Opcode::Add) => self.add().map(|value| self.set(value)),
                Some(Opcode::Multiply) => self.multiply().map(|value| self.set(value)),
                Some(Opcode::Halt) => None,
                None => None,
            }
        } {}

        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.instruction_pointer = 0;
        self
    }

    fn read_at(&self, idx: Address) -> Option<usize> {
        self.memory.get(idx).copied()
    }

    fn set_at(&mut self, idx: Address, value: Value) -> Option<()> {
        self.memory.get_mut(idx).map(|loc| *loc = value)
    }

    fn read_next(&mut self) -> Option<Value> {
        let result = self.memory.get(self.instruction_pointer);
        self.instruction_pointer += 1;
        result.copied()
    }

    fn read_opcode(&mut self) -> Option<Opcode> {
        self.read_next().map(Opcode::from)
    }

    fn read_value(&mut self) -> Option<Value> {
        self.read_next().and_then(|address| self.read_at(address))
    }

    fn set(&mut self, value: Value) -> Option<()> {
        self.read_next()
            .map(|address| self.set_at(address, value))?
    }

    /// Opcode operations.
    fn add(&mut self) -> Option<Value> {
        Some(self.read_value()? + self.read_value()?)
    }

    fn multiply(&mut self) -> Option<Value> {
        Some(self.read_value()? * self.read_value()?)
    }
}
