use crate::{
    chunk::{Chunk, OpCode},
    debug::disassemble_instruction,
    value::Value,
};

pub enum InterpretResult {
    InterpretOK,
    InterpretCompileError,
    InterpretRuntimeError,
}

const STACK_MAX: usize = 256;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: [Value; STACK_MAX],
    stack_top: usize,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: [0.0; STACK_MAX],
            stack_top: 0,
        }
    }

    pub fn interpret(self) {
        self.run();
    }

    fn reset_stack(mut self) {
        self.stack_top = 0;
    }

    fn push(&mut self, value: Value) {
        if self.stack_top + 1 == STACK_MAX {
            panic!("Stack is full. Cannot push value {}", value)
        }
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        if self.stack_top == 0 {
            panic!("Nothing left to pop on the stack")
        }
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn run(mut self) -> InterpretResult {
        loop {
            #[cfg(debug_assertions)]
            {
                if self.stack_top == 0 {
                    println!("[]");
                } else {
                    for i in 0..self.stack_top {
                        print!("[{}]", self.stack[i]);
                    }
                    println!();
                }
                disassemble_instruction(&self.chunk, self.ip);
            }

            match OpCode::try_from(self.chunk.code[self.ip]) {
                Ok(instruction) => {
                    self.ip += 1;

                    match instruction {
                        OpCode::Constant => {
                            let constant = self.chunk.constants[self.chunk.code[self.ip] as usize];
                            self.push(constant);
                            self.ip += 1;
                        }
                        OpCode::Return => {
                            println!("{}", self.pop());
                            return InterpretResult::InterpretOK;
                        }
                    }
                }
                Err(_) => {
                    return InterpretResult::InterpretRuntimeError;
                }
            }
        }
    }
}
