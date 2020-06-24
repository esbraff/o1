use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::error::{Error};

use crate::error::{UnknownOpcodeError};
use crate::opcodes;

pub struct VMMemory(pub Vec<i32>);

impl Deref for VMMemory {
  type Target = Vec<i32>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for VMMemory {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Index<i32> for VMMemory {
  type Output = i32;

  fn index(&self, index: i32) -> &Self::Output {
    &self.0[index as usize]
  }
}

impl IndexMut<i32> for VMMemory {
  fn index_mut(&mut self, index: i32) -> &mut i32 {
    &mut self.0[index as usize]
  }
}

pub struct VM {
  mem: VMMemory
}

impl VM {
  pub fn new(mem: VMMemory) -> Self {
    return Self {
      mem: mem
    }
  }

  pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
    loop {
      let op = self.mem[self.mem[0]];

      match op {
        opcodes::OP_NOP => (),
        _ => return Err(Box::new(UnknownOpcodeError::new(op)))
      }
    }
  }
}