use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::error::{Error};

use crate::error::{UnknownOpcodeError, IntegerOverflowError};
use crate::op;

pub struct VMMemory(pub Vec<i32>);

const MP_INDEX: i32 = 0;

macro_rules! deref {
    ($vm:expr, $i:expr) => {
    $vm.mem[$i]
    };
}

macro_rules! double_deref {
    ($vm:expr, $i:expr) => {
    deref!($vm, deref!($vm, $i))
    };
}

macro_rules! triple_deref {
    ($vm:expr, $i:expr) => {
    deref!($vm, double_deref!($vm, $i))
    };
}

macro_rules! mp {
    ($vm:expr) => {
    deref!($vm, MP_INDEX)
    };
}

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
  fn index_mut(&mut self, index: i32) -> &mut Self::Output {
    &mut self.0[index as usize]
  }
}

pub struct VM {
  pub mem: VMMemory
}

impl VM {
  pub fn new(mem: VMMemory) -> Self {
    return Self { mem }
  }

  pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
    use op::*;

    loop {
      let mp: i32 = mp!(self);
      let op: i32 = deref!(self, mp);

      match op {
        NOP => (),
        ADD_VVA | ADD_AAA | ADD_AVA | ADD_DDA | ADD_DAA | ADD_DVA |
        ADD_VVD | ADD_AAD | ADD_AVD | ADD_DDD | ADD_DAD | ADD_DVD => {
          let (a, b): (i32, i32) = match op {
            ADD_VVA | ADD_VVD => (deref!(self, mp + 1), deref!(self, mp + 2)),
            ADD_AAA | ADD_AAD => (double_deref!(self, mp + 1), double_deref!(self, mp + 2)),
            ADD_AVA | ADD_AVD => (double_deref!(self, mp + 1), deref!(self, mp + 2)),
            ADD_DDA | ADD_DDD => (triple_deref!(self, mp + 1), triple_deref!(self, mp + 2)),
            ADD_DAA | ADD_DAD => (triple_deref!(self, mp + 1), double_deref!(self, mp + 2)),
            ADD_DVA | ADD_DVD => (triple_deref!(self, mp + 1), deref!(self, mp + 2)),
            _ => unimplemented!()
          };

          let sum: Option<i32> = a.checked_add(b);

          if sum.is_none() {
            return Err(Box::new(IntegerOverflowError {
              expr: format!("{} + {}", a, b)
            }));
          }

          if op <= ADD_DVA {
            double_deref!(self, mp + 3) = sum.unwrap();
          } else {
            triple_deref!(self, mp + 3) = sum.unwrap();
          }

          mp!(self) += 4;
        }
        _ => return Err(Box::new(UnknownOpcodeError { opcode: op }))
      }
    }
  }
}