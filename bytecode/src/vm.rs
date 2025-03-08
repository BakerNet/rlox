use std::collections::HashMap;

use crate::{Chunk, Error, OpCode, Value, chunk::long_index, compiler::Compiler, value::ValueVec};

static MAX_STACK: usize = 256;

pub struct VM<'a> {
    globals: HashMap<&'a str, Value<'a>>,
}

impl Default for VM<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self, chunk: Chunk<'a>) -> Result<(), Error> {
        let vmi = VMInterpreter {
            stack: Vec::with_capacity(MAX_STACK),
        };
        vmi.run(&chunk, &mut self.globals)
    }

    pub(crate) fn interpret(&mut self, source: &'a str) -> Result<(), Error> {
        let mut chunk = Chunk::new();
        if !Compiler::compile(source, &mut chunk) {
            return Err(Error::Compiler);
        }
        self.run(chunk)
    }
}

macro_rules! peek {
    ($self:ident, $idx:expr) => {
        $self.stack[$self.stack.len() - $idx - 1]
    };
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
    ($chunk:ident, $idx:expr) => {
        $chunk.code[$idx]
    };
}

macro_rules! binary_op {
    ($self:ident, $chunk:ident, $op:ident, $ip:ident) => {{
        if !matches!(peek!($self, 0), Value::Number(_))
            || !matches!(peek!($self, 1), Value::Number(_))
        {
            $self.print_error($chunk, "Operands must be numbers.", $ip);
            return Err(Error::Runtime);
        }
        let b = pop!($self);
        let a = pop!($self);
        let res = a.$op(&b);
        push!($self, res);
    }};
}

macro_rules! binary_op_supp_str {
    ($self:ident, $chunk:ident, $op:ident, $ip:ident) => {{
        let none_are_string = !matches!(peek!($self, 0), Value::String(_) | Value::ConstString(_))
            && !matches!(peek!($self, 1), Value::String(_) | Value::ConstString(_));
        let not_both_numbers = !matches!(peek!($self, 0), Value::Number(_))
            || !matches!(peek!($self, 1), Value::Number(_));
        if none_are_string && not_both_numbers {
            $self.print_error($chunk, "Operands must be numbers.", $ip);
            return Err(Error::Runtime);
        }
        let b = pop!($self);
        let a = pop!($self);
        let res = a.$op(&b);
        push!($self, res);
    }};
}

struct VMInterpreter<'a> {
    stack: Vec<Value<'a>>,
}

impl<'a> VMInterpreter<'a> {
    fn run(
        mut self,
        chunk: &Chunk<'a>,
        globals: &mut HashMap<&'a str, Value<'a>>,
    ) -> Result<(), Error> {
        let mut ip = 0;
        loop {
            #[cfg(debug_assertions)]
            {
                println!("          {}", ValueVec(&self.stack));
                let _ = chunk.dissassemble_instruction(ip);
            }
            match OpCode::from(read!(chunk, ip)) {
                OpCode::Return => {
                    return Ok(());
                }
                OpCode::Constant => {
                    let value = chunk.read_constant(read!(chunk, ip + 1) as usize);
                    push!(self, value.to_owned());
                    ip += 1;
                }
                OpCode::ConstantLong => {
                    let value = chunk
                        .read_constant(long_index(read!(chunk, ip + 1), read!(chunk, ip + 2)))
                        .to_owned();
                    println!("{}", value);
                    push!(self, value);
                    ip += 2;
                }
                OpCode::Negate => {
                    if !matches!(peek!(self, 0), Value::Number(_)) {
                        self.print_error(chunk, "Operand must be a number.", ip);
                        return Err(Error::Runtime);
                    }
                    let value = pop!(self);
                    push!(self, value.negate());
                }
                OpCode::Add => {
                    binary_op_supp_str!(self, chunk, add, ip);
                }
                OpCode::Subtract => {
                    binary_op!(self, chunk, subtract, ip);
                }
                OpCode::Multiply => {
                    binary_op!(self, chunk, multiply, ip);
                }
                OpCode::Divide => {
                    binary_op!(self, chunk, divide, ip);
                }
                OpCode::Nil => {
                    push!(self, Value::Nil);
                }
                OpCode::True => {
                    push!(self, Value::Bool(true));
                }
                OpCode::False => {
                    push!(self, Value::Bool(false));
                }
                OpCode::Not => {
                    let value = pop!(self);
                    push!(self, Value::Bool(!value.is_truthy()))
                }
                OpCode::Equal => {
                    let b = pop!(self);
                    let a = pop!(self);
                    let res = a == b;
                    push!(self, Value::Bool(res));
                }
                OpCode::Greater => {
                    binary_op!(self, chunk, greater, ip)
                }
                OpCode::Less => {
                    binary_op!(self, chunk, less, ip)
                }
                OpCode::Print => {
                    let value = pop!(self);
                    println!("{}", value);
                }
                OpCode::Pop => {
                    pop!(self);
                }
                OpCode::DefineGlobal => {
                    let name = chunk.read_constant(read!(chunk, ip + 1) as usize).as_str();
                    println!("Defining: {}", name);
                    globals.insert(name, pop!(self));
                    ip += 1;
                }
                OpCode::GetGlobal => {
                    let name = chunk.read_constant(read!(chunk, ip + 1) as usize).as_str();
                    println!("Getting: {}", name);
                    let val = globals.get(name).ok_or_else(|| {
                        self.print_error(chunk, &format!("Undefined variable {}", name), ip);
                        Error::Runtime
                    })?;
                    push!(self, val.clone());
                    ip += 1;
                }
                OpCode::SetGlobal => {
                    let name = chunk.read_constant(read!(chunk, ip + 1) as usize).as_str();
                    println!("Checking: {}", name);
                    globals.get(name).ok_or_else(|| {
                        self.print_error(chunk, &format!("Undefined variable {}", name), ip);
                        Error::Runtime
                    })?;
                    println!("Setting: {}", name);
                    globals.insert(name, peek!(self, 0).clone());
                    ip += 1;
                }
                OpCode::Unknown => todo!(),
            };
            ip += 1;
        }
    }

    fn print_error(&self, chunk: &Chunk<'a>, message: &str, ip: usize) {
        eprintln!("{} [line {}] in script", message, chunk.read_line(ip));
    }
}
