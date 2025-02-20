use std::fmt::Display;

use crate::value::Value;

pub enum OpCode {
    Return,
    Constant,
    ConstantLong,
    Unknown,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Return,
            1 => Self::Constant,
            2 => Self::ConstantLong,
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
                OpCode::Constant => "OP_CONSTANT",
                OpCode::ConstantLong => "OP_CONSTANT_LONG",
                OpCode::Return => "OP_RETURN",
                OpCode::Unknown => "UNKNOWN",
            }
        )
    }
}

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
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
            let const_idx_top = const_idx >> 8;
            let const_idx_bot = const_idx & 255;
            self.write(const_idx_top as u8, line);
            self.write(const_idx_bot as u8, line);
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
            print!("{cursor:04} ",);
            if cursor > 0 && self.lines[cursor] == self.lines[cursor - 1] {
                print!("   | ");
            } else {
                print!("{:4} ", self.lines[cursor]);
            }
            cursor = match OpCode::from(self.code[cursor]) {
                OpCode::Constant => {
                    let const_idx = self.code[cursor + 1] as usize;
                    self.print_constant(const_idx, cursor)
                }
                OpCode::ConstantLong => {
                    let const_idx_top = self.code[cursor + 1] as usize;
                    let const_idx_bot = self.code[cursor + 2] as usize;
                    let const_idx = const_idx_top << 8 | const_idx_bot;
                    self.print_constant(const_idx, cursor)
                }
                OpCode::Return => self.print_simple(OpCode::Return, cursor),
                OpCode::Unknown => {
                    println!("Unknown OpCode: {}", self.code[cursor]);
                    cursor + 1
                }
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
}
