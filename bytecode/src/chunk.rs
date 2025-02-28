use std::fmt::Display;

use crate::value::Value;

pub enum OpCode {
    Return,
    Constant,
    ConstantLong,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Unknown,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Return,
            1 => Self::Constant,
            2 => Self::ConstantLong,
            3 => Self::Negate,
            4 => Self::Add,
            5 => Self::Subtract,
            6 => Self::Multiply,
            7 => Self::Divide,
            _ => Self::Unknown,
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::Return => 0,
            OpCode::Constant => 1,
            OpCode::ConstantLong => 2,
            OpCode::Negate => 3,
            OpCode::Add => 4,
            OpCode::Subtract => 5,
            OpCode::Multiply => 6,
            OpCode::Divide => 7,
            OpCode::Unknown => 255,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpCode::Return => "OP_RETURN",
                OpCode::Constant => "OP_CONSTANT",
                OpCode::ConstantLong => "OP_CONSTANT_LONG",
                OpCode::Negate => "OP_NEGATE",
                OpCode::Add => "OP_ADD",
                OpCode::Subtract => "OP_SUBTRACT",
                OpCode::Multiply => "OP_MULTIPLY",
                OpCode::Divide => "OP_DIVIDE",
                OpCode::Unknown => "UNKNOWN",
            }
        )
    }
}

pub fn long_index(idx_top: u8, idx_bot: u8) -> usize {
    (idx_top as usize) << 8 | idx_bot as usize
}

pub fn break_index(idx: usize) -> [u8; 2] {
    [(idx >> 8) as u8, (idx & 255) as u8]
}

pub struct Chunk {
    pub(crate) code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        let const_idx = self.add_constant(value);
        if const_idx < 256 {
            self.write(OpCode::Constant.into(), line);
            self.write(const_idx as u8, line);
        } else {
            self.write(OpCode::ConstantLong.into(), line);
            let [const_idx_top, const_idx_bot] = break_index(const_idx);
            self.write(const_idx_top, line);
            self.write(const_idx_bot, line);
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn free(self) {
        drop(self);
    }

    pub fn dissassemble(&self, name: &str) {
        println!("== {name} ==");

        let mut cursor = 0;
        while cursor < self.code.len() {
            cursor = self.dissassemble_instruction(cursor)
        }
    }

    pub fn dissassemble_instruction(&self, index: usize) -> usize {
        print!("{index:04} ",);
        if index > 0 && self.lines[index] == self.lines[index - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[index]);
        }
        match OpCode::from(self.code[index]) {
            OpCode::Return => self.print_simple(OpCode::Return, index),
            OpCode::Constant => {
                let const_idx = self.code[index + 1] as usize;
                self.print_constant(const_idx, index)
            }
            OpCode::ConstantLong => {
                let const_idx = long_index(self.code[index + 1], self.code[index + 2]);
                self.print_constant_long(const_idx, index)
            }
            OpCode::Negate => self.print_simple(OpCode::Negate, index),
            OpCode::Add => self.print_simple(OpCode::Add, index),
            OpCode::Subtract => self.print_simple(OpCode::Subtract, index),
            OpCode::Multiply => self.print_simple(OpCode::Multiply, index),
            OpCode::Divide => self.print_simple(OpCode::Divide, index),
            OpCode::Unknown => {
                println!("Unknown OpCode: {}", self.code[index]);
                index + 1
            }
        }
    }

    fn print_simple(&self, op: OpCode, cursor: usize) -> usize {
        println!("{op}");
        cursor + 1
    }

    fn print_constant(&self, const_idx: usize, cursor: usize) -> usize {
        println!(
            "{:16} {:4} '{}'",
            OpCode::Constant,
            const_idx,
            self.constants[const_idx]
        );
        cursor + 2
    }

    fn print_constant_long(&self, const_idx: usize, cursor: usize) -> usize {
        println!(
            "{:16} {:4} '{}'",
            OpCode::ConstantLong,
            const_idx,
            self.constants[const_idx]
        );
        cursor + 3
    }

    pub(crate) fn read_constant(&self, index: usize) -> &Value {
        &self.constants[index]
    }
}
