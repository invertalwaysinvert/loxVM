use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
    println!("===================\n");
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset])
    }
    match OpCode::try_from(chunk.code[offset]) {
        Ok(instruction) => match instruction {
            OpCode::Constant => constant_instruction("OP_CONSTANT", chunk, offset),
            OpCode::Add => simple_instruction("OP_ADD", offset),
            OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
            OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
            OpCode::Negate => simple_instruction("OP_NEGATE", offset),
            OpCode::Return => simple_instruction("OP_RETURN", offset),
            OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
        },
        Err(_) => {
            println!("Unknown opcode {}", chunk.code[offset]);
            offset + 1
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    println!(
        "{:.16} {:4} {}",
        name, constant, chunk.constants[constant as usize]
    );
    offset + 2
}
