use bytecode::{Chunk, OpCode, Value};

pub fn main() {
    let mut chunk = Chunk::new();
    let const_idx = chunk.add_constant(Value::Number(1.2));
    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(const_idx as u8, 123);
    chunk.write(OpCode::Return.into(), 123);
    chunk.dissassemble("test chunk");
}
