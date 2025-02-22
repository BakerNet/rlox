use bytecode::{Chunk, OpCode, VM, Value};

pub fn main() {
    let mut chunk = Chunk::new();
    chunk.write_constant(Value::Number(1.2), 123);
    chunk.write_constant(Value::Number(3.4), 123);
    chunk.write(OpCode::Add.into(), 123);
    chunk.write_constant(Value::Number(5.6), 123);
    chunk.write(OpCode::Divide.into(), 123);
    chunk.write(OpCode::Negate.into(), 123);
    chunk.write(OpCode::Return.into(), 123);
    chunk.dissassemble("test chunk");
    println!();

    println!("Running");
    let vm = VM::new();
    let _ = vm.run(chunk);
}
