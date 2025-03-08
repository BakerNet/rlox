use crate::{Chunk, Error, OpCode, Value, chunk::long_index, compiler::Compiler, value::ValueVec};

static MAX_STACK: usize = 256;

pub struct VM {}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, chunk: Chunk) -> Result<(), Error> {
        VMInterpreter {
            chunk,
            stack: Vec::with_capacity(MAX_STACK),
        }
        .run()
    }

    pub(crate) fn interpret(&self, source: String) -> Result<(), Error> {
        let mut chunk = Chunk::new();
        if !Compiler::compile(&source, &mut chunk) {
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
    ($self:ident, $idx:expr) => {
        $self.chunk.code[$idx]
    };
}

macro_rules! binary_op {
    ($self:ident, $op:ident, $ip:ident) => {{
        if !matches!(peek!($self, 0), Value::Number(_))
            || !matches!(peek!($self, 1), Value::Number(_))
        {
            $self.print_error("Operands must be numbers.", $ip);
            return Err(Error::Runtime);
        }
        let b = pop!($self);
        let a = pop!($self);
        let res = a.$op(&b);
        push!($self, res);
    }};
}

macro_rules! binary_op_supp_str {
    ($self:ident, $op:ident, $ip:ident) => {{
        let none_are_string = !matches!(peek!($self, 0), Value::String(_) | Value::ConstString(_))
            && !matches!(peek!($self, 1), Value::String(_) | Value::ConstString(_));
        let not_both_numbers = !matches!(peek!($self, 0), Value::Number(_))
            || !matches!(peek!($self, 1), Value::Number(_));
        if none_are_string && not_both_numbers {
            $self.print_error("Operands must be numbers.", $ip);
            return Err(Error::Runtime);
        }
        let b = pop!($self);
        let a = pop!($self);
        let res = a.$op(&b);
        push!($self, res);
    }};
}

struct VMInterpreter<'a> {
    chunk: Chunk<'a>,
    stack: Vec<Value<'a>>,
}

impl<'a> VMInterpreter<'a> {
    fn run(&'a mut self) -> Result<(), Error> {
        let mut ip = 0;
        loop {
            #[cfg(debug_assertions)]
            {
                println!("          {}", ValueVec(&self.stack));
                let _ = self.chunk.dissassemble_instruction(ip);
            }
            match OpCode::from(read!(self, ip)) {
                OpCode::Return => {
                    println!("{}", pop!(self));
                    break;
                }
                OpCode::Constant => {
                    let value = self.chunk.read_constant(read!(self, ip + 1) as usize);
                    push!(self, value.to_owned());
                    println!("{}", value);
                    ip += 1;
                }
                OpCode::ConstantLong => {
                    let value = self
                        .chunk
                        .read_constant(long_index(read!(self, ip + 1), read!(self, ip + 2)))
                        .to_owned();
                    println!("{}", value);
                    push!(self, value);
                    ip += 2;
                }
                OpCode::Negate => {
                    if !matches!(peek!(self, 0), Value::Number(_)) {
                        self.print_error("Operand must be a number.", ip);
                        return Err(Error::Runtime);
                    }
                    let value = pop!(self);
                    push!(self, value.negate());
                }
                OpCode::Add => {
                    binary_op_supp_str!(self, add, ip);
                }
                OpCode::Subtract => {
                    binary_op!(self, subtract, ip);
                }
                OpCode::Multiply => {
                    binary_op!(self, multiply, ip);
                }
                OpCode::Divide => {
                    binary_op!(self, divide, ip);
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
                    binary_op!(self, greater, ip)
                }
                OpCode::Less => {
                    binary_op!(self, less, ip)
                }
                OpCode::Unknown => todo!(),
            };
            ip += 1;
        }
        Ok(())
    }

    fn print_error(&self, message: &str, ip: usize) {
        eprintln!("{} [line {}] in script", message, self.chunk.read_line(ip));
    }
}
