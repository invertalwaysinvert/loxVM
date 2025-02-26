use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;

mod chunk;
mod debug;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn as u8);
    disassemble_chunk(&chunk, "test chunk");
    chunk.free();
}
