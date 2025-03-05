use chunk::{Chunk, OpCode};
use debug::disassemble_chunk;
use vm::VM;

mod chunk;
mod debug;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(constant, 123);
    chunk.write(OpCode::Negate as u8, 123);
    let constant = chunk.add_constant(432.0);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(constant, 123);
    chunk.write(OpCode::Divide as u8, 123);
    chunk.write(OpCode::Return as u8, 123);
    #[cfg(debug_assertions)]
    disassemble_chunk(&chunk, "test chunk");
    let mut vm = VM::new(chunk);
    vm.interpret();
}
