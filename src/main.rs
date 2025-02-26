use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

mod chunk;
mod debug;
mod value;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(constant, 123);
    chunk.write(OpCode::Return as u8, 123);
    disassemble_chunk(&chunk, "test chunk");
    chunk.free();
}
