use std::fmt::{Display, Formatter, Result};
use std::error::{Error};

#[derive(Debug)]
pub struct UnknownOpcodeError {
  pub opcode: i32
}

impl UnknownOpcodeError {
  pub fn new(opcode: i32) -> Self {
    UnknownOpcodeError {
      opcode: opcode
    }
  }
}

impl Display for UnknownOpcodeError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", format!("Unknown opcode {}", self.opcode))
  }
}

impl Error for UnknownOpcodeError { }