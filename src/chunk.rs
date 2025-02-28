use crate::value::Value;

pub enum OpCode {
    Return = 1,
    Constant = 2,
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OpCode::Return),
            2 => Ok(OpCode::Constant),
            _ => Err("Invalid OpCode found"),
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn free(&mut self) {
        self.code.clear();
        self.constants.clear();
        self.lines.clear();
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        match (self.constants.len() - 1).try_into() {
            Ok(len) => len,
            Err(_) => panic!("Reached constant limit"),
        }
    }
}
