use std::fmt::{Display, Formatter, Result};
use std::error::{Error};

#[derive(Debug)]
pub struct UnknownOpcodeError {
  pub opcode: i32
}

impl Display for UnknownOpcodeError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", format!("Unknown opcode {}", self.opcode))
  }
}

impl Error for UnknownOpcodeError { }

#[derive(Debug)]
pub struct IntegerOverflowError {
  pub expr: String
}

impl Display for IntegerOverflowError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", format!("Integer overflow in expression {}", self.expr))
  }
}

impl Error for IntegerOverflowError { }