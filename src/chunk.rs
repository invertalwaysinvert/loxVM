pub enum OpCode {
    OpReturn = 1,
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OpCode::OpReturn),
            _ => Err("Invalid OpCode found"),
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new() }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn free(&mut self) {
        self.code.clear();
    }
}
