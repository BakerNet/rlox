use crate::{
    Chunk, Error, OpCode, Value,
    chunk::long_index,
    scan::{Scanner, Token, TokenType},
    value::ValueVec,
};

static MAX_STACK: usize = 256;

pub struct VM {}

impl VM {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, chunk: Chunk) -> Result<(), Error> {
        return VMInterpreter {
            chunk,
            stack: Vec::with_capacity(MAX_STACK),
        }
        .run();
    }

    pub(crate) fn interpret(&self, source: String) -> Result<(), Error> {
        self.compile(source)?;
        todo!()
    }

    pub(crate) fn compile(&self, source: String) -> Result<(), Error> {
        let mut scanner = Scanner::new(&source);
        let mut line = 0;
        loop {
            let token = scanner.scan_token();
            if token.line != line {
                line = token.line;
                print!("{line:4}");
            } else {
                print!("  | ")
            }
            println!("{:10} {}", token.ttype, token.lexeme);

            if matches!(token.ttype, TokenType::EoF) {
                break;
            }
        }
        Ok(())
    }
}

macro_rules! pop {
    ($self:ident) => {
        $self.stack.pop().unwrap()
    };
}

macro_rules! push {
    ($self:ident, $value:ident) => {
        $self.stack.push($value)
    };
    ($self:ident, $value:expr) => {
        $self.stack.push($value)
    };
}

macro_rules! read {
    ($self:ident, $idx:expr) => {
        $self.chunk.code[$idx]
    };
}

macro_rules! binary_op {
    ($self:ident, $op:ident) => {{
        let b = pop!($self);
        let a = pop!($self);
        let res = a.$op(&b);
        push!($self, res);
    }};
}

struct VMInterpreter {
    chunk: Chunk,
    stack: Vec<Value>,
}

impl VMInterpreter {
    fn run(&mut self) -> Result<(), Error> {
        let mut ip = 0;
        loop {
            #[cfg(debug_assertions)]
            {
                println!("          {}", ValueVec(&self.stack));
                let _ = self.chunk.dissassemble_instruction(ip);
            }
            let _instruction = match OpCode::from(read!(self, ip)) {
                OpCode::Return => {
                    println!("{}", pop!(self));
                    break;
                }
                OpCode::Constant => {
                    let value = self.chunk.read_constant(read!(self, ip + 1) as usize);
                    push!(self, value.to_owned());
                    println!("{}", value);
                    ip += 2;
                }
                OpCode::ConstantLong => {
                    let value = self
                        .chunk
                        .read_constant(long_index(read!(self, ip + 1), read!(self, ip + 2)))
                        .to_owned();
                    println!("{}", value);
                    push!(self, value);
                    ip += 3;
                }
                OpCode::Negate => {
                    let value = pop!(self);
                    push!(self, value.negate());
                    ip += 1;
                }
                OpCode::Add => {
                    binary_op!(self, add);
                    ip += 1;
                }
                OpCode::Subtract => {
                    binary_op!(self, subtract);
                    ip += 1;
                }
                OpCode::Multiply => {
                    binary_op!(self, multiply);
                    ip += 1;
                }
                OpCode::Divide => {
                    binary_op!(self, divide);
                    ip += 1;
                }
                OpCode::Unknown => todo!(),
            };
        }
        Ok(())
    }
}
